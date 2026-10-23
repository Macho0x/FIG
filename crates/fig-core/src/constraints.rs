//! Runtime checks for FSL `decimal64` and `string` constraints.
//!
//! The 8-byte wire slot stays IEEE-754 `f64`. Decoders reject non-finite
//! values, values that are not on the declared scale, scaled magnitudes that
//! do not fit in `i64`, and values outside an optional min/max. Encoders
//! round off-scale values onto the scale and leave bits unchanged when the
//! value is already within a fraction of a tick.

/// Absolute distance, in ticks, that still counts as on-scale.
///
/// Float dust from `value * 10^precision` is far inside this window.
/// Half a tick (0.5) is not.
const SCALE_EPS: f64 = 1e-3;

/// Reject `value` unless it already lies on `precision` and inside bounds.
///
/// The returned bits are `value` itself when the check passes, so an on-scale
/// input round-trips without being rewritten.
pub fn check_decimal(
    value: f64,
    precision: u8,
    min: Option<f64>,
    max: Option<f64>,
) -> Result<f64, String> {
    let rounded = scaled_round(value, precision)?;
    let scale = scale_factor(precision)?;
    let scaled = value * scale;
    if (scaled - rounded).abs() > SCALE_EPS {
        return Err(format!(
            "decimal {value} is off scale (precision {precision})"
        ));
    }
    check_bounds(value, min, max)?;
    Ok(value)
}

/// Round `value` onto `precision` when it is off-scale, then apply bounds.
///
/// Values already within [`SCALE_EPS`] of a tick keep their original bits.
pub fn quantize_decimal(
    value: f64,
    precision: u8,
    min: Option<f64>,
    max: Option<f64>,
) -> Result<f64, String> {
    let rounded = scaled_round(value, precision)?;
    let scale = scale_factor(precision)?;
    let scaled = value * scale;
    let out = if (scaled - rounded).abs() <= SCALE_EPS {
        value
    } else {
        rounded / scale
    };
    check_bounds(out, min, max)?;
    Ok(out)
}

/// Reject text that exceeds `max_len`, contains a lowercase letter when
/// `uppercase` is set, or fails `pattern`.
///
/// `pattern` is the FSL character-class form `[class]+`. The schema lexer
/// keeps backslashes, so `\\-` and `\-` both mean a literal `-`.
pub fn check_text(
    value: &str,
    max_len: Option<usize>,
    pattern: Option<&str>,
    uppercase: bool,
) -> Result<(), String> {
    if let Some(max_len) = max_len {
        let len = value.chars().count();
        if len > max_len {
            return Err(format!("text length {len} exceeds max_len {max_len}"));
        }
    }
    if uppercase && value.chars().any(|c| c.is_lowercase()) {
        return Err("text must be uppercase".into());
    }
    if let Some(pattern) = pattern {
        if !match_char_class(value, pattern) {
            return Err(format!("text does not match pattern {pattern}"));
        }
    }
    Ok(())
}

fn scaled_round(value: f64, precision: u8) -> Result<f64, String> {
    if !value.is_finite() {
        return Err("decimal is not finite".into());
    }
    let scale = scale_factor(precision)?;
    let scaled = value * scale;
    if !scaled.is_finite() {
        return Err("decimal scale overflow".into());
    }
    let rounded = scaled.round();
    if rounded.abs() > i64::MAX as f64 {
        return Err("decimal exceeds i64 scale".into());
    }
    Ok(rounded)
}

fn scale_factor(precision: u8) -> Result<f64, String> {
    if precision > 18 {
        return Err("decimal precision too large".into());
    }
    Ok(10f64.powi(i32::from(precision)))
}

fn check_bounds(value: f64, min: Option<f64>, max: Option<f64>) -> Result<(), String> {
    if let Some(min) = min {
        if value < min {
            return Err(format!("decimal {value} is below min {min}"));
        }
    }
    if let Some(max) = max {
        if value > max {
            return Err(format!("decimal {value} is above max {max}"));
        }
    }
    Ok(())
}

/// Match `[class]+`. An empty value fails the `+`. Any other shape fails closed.
fn match_char_class(value: &str, pattern: &str) -> bool {
    let Some(class) = pattern.strip_prefix('[').and_then(|s| s.strip_suffix("]+")) else {
        return false;
    };
    if value.is_empty() {
        return false;
    }
    let ranges = parse_class(class);
    value.chars().all(|ch| char_in_class(ch, &ranges))
}

fn parse_class(class: &str) -> Vec<(char, char)> {
    let chars: Vec<char> = class.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' {
            // FSL stores `\\-` (two source backslashes) for a literal dash.
            // `\-` is the same escape after one unescape pass.
            if i + 2 < chars.len() && chars[i + 1] == '\\' {
                out.push((chars[i + 2], chars[i + 2]));
                i += 3;
                continue;
            }
            if i + 1 < chars.len() {
                out.push((chars[i + 1], chars[i + 1]));
                i += 2;
                continue;
            }
        }
        if i + 2 < chars.len() && chars[i + 1] == '-' && chars[i + 2] != '\\' {
            out.push((chars[i], chars[i + 2]));
            i += 3;
            continue;
        }
        out.push((chars[i], chars[i]));
        i += 1;
    }
    out
}

fn char_in_class(ch: char, ranges: &[(char, char)]) -> bool {
    ranges.iter().any(|&(a, b)| ch >= a && ch <= b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_scale_keeps_bits() {
        let v = check_decimal(100.0, 8, Some(0.0), None).unwrap();
        assert_eq!(v.to_bits(), 100.0f64.to_bits());
        let q = quantize_decimal(50000.0, 8, Some(0.0), None).unwrap();
        assert_eq!(q.to_bits(), 50000.0f64.to_bits());
    }

    #[test]
    fn dust_passes_half_tick_fails() {
        assert!(check_decimal(0.1, 8, None, None).is_ok());
        assert!(check_decimal(1.5, 0, Some(0.0), Some(999_999_999.0)).is_err());
        assert_eq!(
            quantize_decimal(1.5, 0, Some(0.0), Some(999_999_999.0)).unwrap(),
            2.0
        );
    }

    #[test]
    fn rejects_non_finite_and_bounds() {
        assert!(check_decimal(f64::NAN, 8, Some(0.0), None).is_err());
        assert!(check_decimal(-1.0, 8, Some(0.0), None).is_err());
        assert!(check_decimal(-1.25, 8, None, None).is_ok());
        assert!(check_decimal(1.0e19, 0, None, None).is_err());
    }

    #[test]
    fn client_order_id_pattern_and_symbol_case() {
        let pattern = "[A-Za-z0-9\\\\-]+";
        assert!(check_text("ORD-001", Some(20), Some(pattern), false).is_ok());
        assert!(check_text("ORD_001", Some(20), Some(pattern), false).is_err());
        assert!(check_text("", Some(20), Some(pattern), false).is_err());
        assert!(check_text("AAPL", Some(32), None, true).is_ok());
        assert!(check_text("Aapl", Some(32), None, true).is_err());
        assert!(check_text(&"A".repeat(33), Some(32), None, true).is_err());
    }
}
