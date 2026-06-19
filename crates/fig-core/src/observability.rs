//! Observability — OpenTelemetry-compatible tracing spans and atomic metrics.
//!
//! This module provides lightweight instrumentation for FIG protocol operations.
//! Traces use the [`tracing`] crate for span creation; metrics use lock-free
//! [`AtomicU64`] counters stored in a global [`Metrics`] singleton.
//!
//! # Integration
//!
//! External gateways or binaries wire [`tracing-subscriber`] to export spans to
//! OpenTelemetry, and periodically drain [`METRICS`] snapshots into Prometheus
//! or other backends. The core library itself stays dependency-light.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::LazyLock;

use tracing::{debug_span, info_span, Span};

// ---------------------------------------------------------------------------
// Tracing spans
// ---------------------------------------------------------------------------

/// Create a tracing span for frame encoding.
pub fn span_encode(channel_id: u16, frame_type: &str) -> Span {
    debug_span!("fig.encode", channel_id = channel_id, frame_type = frame_type)
}

/// Create a tracing span for frame decoding.
pub fn span_decode(channel_id: u16) -> Span {
    debug_span!("fig.decode", channel_id = channel_id)
}

/// Create a tracing span for channel open.
pub fn span_channel_open(channel_id: u16, mode: &str) -> Span {
    info_span!("fig.channel.open", channel_id = channel_id, mode = mode)
}

/// Create a tracing span for channel close.
pub fn span_channel_close(channel_id: u16) -> Span {
    info_span!("fig.channel.close", channel_id = channel_id)
}

/// Create a tracing span for session creation.
pub fn span_session_create(session_id: &str) -> Span {
    info_span!("fig.session.create", session_id = session_id)
}

/// Create a tracing span for session resumption.
pub fn span_session_resume(session_id: &str) -> Span {
    info_span!("fig.session.resume", session_id = session_id)
}

/// Create a tracing span for TREE connection.
pub fn span_connection(remote: &str) -> Span {
    info_span!("fig.connection", remote = remote)
}

/// Create a tracing span for gateway translation (FIX→FIG, REST→FIG, WS→FIG).
pub fn span_gateway_translate(from: &str, to: &str) -> Span {
    debug_span!("fig.gateway.translate", from = from, to = to)
}

// ---------------------------------------------------------------------------
// Metrics
// ---------------------------------------------------------------------------

/// Global metrics counters for FIG protocol operations.
///
/// All counters are lock-free [`AtomicU64`] values. Use the associated helper
/// methods to read and write them.
pub struct Metrics {
    /// Total frames encoded.
    pub frames_encoded: AtomicU64,
    /// Total frames decoded.
    pub frames_decoded: AtomicU64,
    /// Total channels opened.
    pub channels_opened: AtomicU64,
    /// Total channels closed.
    pub channels_closed: AtomicU64,
    /// Total sessions created.
    pub sessions_created: AtomicU64,
    /// Total sessions resumed.
    pub sessions_resumed: AtomicU64,
    /// Total bytes sent.
    pub bytes_sent: AtomicU64,
    /// Total bytes received.
    pub bytes_received: AtomicU64,
    /// Total errors.
    pub errors: AtomicU64,
    /// Total gateway translations (FIX→FIG, REST→FIG, WS→FIG).
    pub gateway_translations: AtomicU64,
}

/// Global metrics singleton.
pub static METRICS: LazyLock<Metrics> = LazyLock::new(|| Metrics {
    frames_encoded: AtomicU64::new(0),
    frames_decoded: AtomicU64::new(0),
    channels_opened: AtomicU64::new(0),
    channels_closed: AtomicU64::new(0),
    sessions_created: AtomicU64::new(0),
    sessions_resumed: AtomicU64::new(0),
    bytes_sent: AtomicU64::new(0),
    bytes_received: AtomicU64::new(0),
    errors: AtomicU64::new(0),
    gateway_translations: AtomicU64::new(0),
});

impl Metrics {
    /// Increment a counter by 1.
    #[inline]
    pub fn inc(counter: &AtomicU64) {
        counter.fetch_add(1, Ordering::SeqCst);
    }

    /// Add a value to a counter.
    #[inline]
    pub fn add(counter: &AtomicU64, value: u64) {
        counter.fetch_add(value, Ordering::SeqCst);
    }

    /// Get the current value of a counter.
    #[inline]
    pub fn get(counter: &AtomicU64) -> u64 {
        counter.load(Ordering::SeqCst)
    }

    /// Reset all counters to zero (for testing).
    pub fn reset(&self) {
        self.frames_encoded.store(0, Ordering::SeqCst);
        self.frames_decoded.store(0, Ordering::SeqCst);
        self.channels_opened.store(0, Ordering::SeqCst);
        self.channels_closed.store(0, Ordering::SeqCst);
        self.sessions_created.store(0, Ordering::SeqCst);
        self.sessions_resumed.store(0, Ordering::SeqCst);
        self.bytes_sent.store(0, Ordering::SeqCst);
        self.bytes_received.store(0, Ordering::SeqCst);
        self.errors.store(0, Ordering::SeqCst);
        self.gateway_translations.store(0, Ordering::SeqCst);
    }

    /// Get a snapshot of all metrics as a HashMap.
    pub fn snapshot(&self) -> HashMap<&'static str, u64> {
        let mut map = HashMap::new();
        map.insert("frames_encoded", Self::get(&self.frames_encoded));
        map.insert("frames_decoded", Self::get(&self.frames_decoded));
        map.insert("channels_opened", Self::get(&self.channels_opened));
        map.insert("channels_closed", Self::get(&self.channels_closed));
        map.insert("sessions_created", Self::get(&self.sessions_created));
        map.insert("sessions_resumed", Self::get(&self.sessions_resumed));
        map.insert("bytes_sent", Self::get(&self.bytes_sent));
        map.insert("bytes_received", Self::get(&self.bytes_received));
        map.insert("errors", Self::get(&self.errors));
        map.insert("gateway_translations", Self::get(&self.gateway_translations));
        map
    }

    /// Render metrics in Prometheus text exposition format.
    pub fn render_prometheus(&self) -> String {
        let snap = self.snapshot();
        let mut out = String::new();
        for (name, value) in snap {
            out.push_str(&format!("# TYPE fig_{name} counter\n"));
            out.push_str(&format!("fig_{name} {value}\n"));
        }
        out
    }
}

/// Initialize a tracing subscriber with env-filter (OpenTelemetry-compatible).
///
/// Wire an OTel exporter by installing `tracing-opentelemetry` in the
/// application binary before calling this, or use the default fmt layer.
pub fn init_tracing(default_filter: &str) {
    use tracing_subscriber::{fmt, EnvFilter};

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(default_filter));

    let _ = fmt()
        .with_env_filter(filter)
        .with_target(false)
        .try_init();
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Create a fresh local Metrics for isolated tests (avoids parallel
    /// test races on the global METRICS singleton).
    fn new_metrics() -> Metrics {
        Metrics {
            frames_encoded: AtomicU64::new(0),
            frames_decoded: AtomicU64::new(0),
            channels_opened: AtomicU64::new(0),
            channels_closed: AtomicU64::new(0),
            sessions_created: AtomicU64::new(0),
            sessions_resumed: AtomicU64::new(0),
            bytes_sent: AtomicU64::new(0),
            bytes_received: AtomicU64::new(0),
            errors: AtomicU64::new(0),
            gateway_translations: AtomicU64::new(0),
        }
    }

    // -- Metrics tests ------------------------------------------------------

    #[test]
    fn test_metrics_increment() {
        let m = new_metrics();

        Metrics::inc(&m.frames_encoded);
        Metrics::inc(&m.frames_encoded);
        Metrics::inc(&m.channels_opened);

        assert_eq!(Metrics::get(&m.frames_encoded), 2);
        assert_eq!(Metrics::get(&m.channels_opened), 1);
        assert_eq!(Metrics::get(&m.channels_closed), 0);
    }

    #[test]
    fn test_metrics_add() {
        let m = new_metrics();

        Metrics::add(&m.bytes_sent, 1500);
        Metrics::add(&m.bytes_received, 800);
        Metrics::add(&m.bytes_sent, 500);

        assert_eq!(Metrics::get(&m.bytes_sent), 2000);
        assert_eq!(Metrics::get(&m.bytes_received), 800);
    }

    #[test]
    fn test_metrics_reset() {
        let m = new_metrics();

        // First, set some values
        Metrics::inc(&m.errors);
        Metrics::inc(&m.errors);
        Metrics::add(&m.bytes_sent, 1024);

        assert_eq!(Metrics::get(&m.errors), 2);
        assert_eq!(Metrics::get(&m.bytes_sent), 1024);

        // Reset
        m.reset();

        assert_eq!(Metrics::get(&m.errors), 0);
        assert_eq!(Metrics::get(&m.bytes_sent), 0);
        assert_eq!(Metrics::get(&m.frames_encoded), 0);
        assert_eq!(Metrics::get(&m.gateway_translations), 0);
    }

    #[test]
    fn test_render_prometheus_format() {
        let m = new_metrics();
        Metrics::inc(&m.frames_encoded);
        Metrics::add(&m.bytes_sent, 100);
        let text = m.render_prometheus();
        assert!(text.contains("# TYPE fig_frames_encoded counter"));
        assert!(text.contains("fig_frames_encoded 1"));
        assert!(text.contains("fig_bytes_sent 100"));
    }

    #[test]
    fn test_metrics_snapshot() {
        let m = new_metrics();

        Metrics::inc(&m.frames_encoded);
        Metrics::inc(&m.sessions_created);
        Metrics::add(&m.bytes_received, 4096);
        Metrics::inc(&m.gateway_translations);
        Metrics::inc(&m.gateway_translations);

        let snap = m.snapshot();

        // Verify all expected keys are present
        assert_eq!(snap.get("frames_encoded"), Some(&1));
        assert_eq!(snap.get("sessions_created"), Some(&1));
        assert_eq!(snap.get("bytes_received"), Some(&4096));
        assert_eq!(snap.get("gateway_translations"), Some(&2));

        // Verify keys with zero values are present
        assert_eq!(snap.get("frames_decoded"), Some(&0));
        assert_eq!(snap.get("channels_opened"), Some(&0));
        assert_eq!(snap.get("channels_closed"), Some(&0));
        assert_eq!(snap.get("sessions_resumed"), Some(&0));
        assert_eq!(snap.get("bytes_sent"), Some(&0));
        assert_eq!(snap.get("errors"), Some(&0));

        // Verify total key count
        assert_eq!(snap.len(), 10);
    }

    #[test]
    fn test_metrics_concurrent() {
        use std::sync::Arc;

        let m = Arc::new(new_metrics());

        let threads: Vec<_> = (0..8)
            .map(|_| {
                let m = Arc::clone(&m);
                std::thread::spawn(move || {
                    for _ in 0..1000 {
                        Metrics::inc(&m.frames_encoded);
                        Metrics::add(&m.bytes_sent, 1);
                    }
                })
            })
            .collect();

        for t in threads {
            t.join().unwrap();
        }

        assert_eq!(Metrics::get(&m.frames_encoded), 8 * 1000);
        assert_eq!(Metrics::get(&m.bytes_sent), 8 * 1000);
    }

    #[test]
    fn test_metrics_global() {
        // Verify the global METRICS is accessible and all counters exist.
        // We only read — no mutation — to avoid races with parallel tests.
        let snap = METRICS.snapshot();
        assert!(snap.contains_key("errors"));
        assert!(snap.contains_key("frames_encoded"));
        assert!(snap.contains_key("frames_decoded"));
        assert!(snap.contains_key("channels_opened"));
        assert!(snap.contains_key("channels_closed"));
        assert!(snap.contains_key("sessions_created"));
        assert!(snap.contains_key("sessions_resumed"));
        assert!(snap.contains_key("bytes_sent"));
        assert!(snap.contains_key("bytes_received"));
        assert!(snap.contains_key("gateway_translations"));
        assert_eq!(snap.len(), 10);

        // The lazy static is initialised: all values are valid u64.
        for (_key, val) in &snap {
            assert!(*val < u64::MAX, "counter overflowed");
        }

        // Helper functions work on the global counters without panic.
        Metrics::get(&METRICS.frames_encoded);
        Metrics::get(&METRICS.errors);
    }

    // -- Span tests ---------------------------------------------------------

    #[test]
    fn test_span_creation() {
        // Verify each span function creates a valid tracing Span with correct
        // metadata (name and fields). Spans may be disabled when no subscriber
        // is active, but metadata is always present.

        // Helper: assert span has correct name.
        fn assert_span_name(span: &Span, expected: &str) {
            let meta = span.metadata().expect("span must have metadata");
            assert_eq!(meta.name(), expected, "span name mismatch");
        }

        let s1 = span_encode(7, "Request");
        assert_span_name(&s1, "fig.encode");

        let s2 = span_decode(3);
        assert_span_name(&s2, "fig.decode");

        let s3 = span_channel_open(1, "stream");
        assert_span_name(&s3, "fig.channel.open");

        let s4 = span_channel_close(1);
        assert_span_name(&s4, "fig.channel.close");

        let s5 = span_session_create("sess-abc123");
        assert_span_name(&s5, "fig.session.create");

        let s6 = span_session_resume("sess-abc123");
        assert_span_name(&s6, "fig.session.resume");

        let s7 = span_connection("127.0.0.1:8443");
        assert_span_name(&s7, "fig.connection");

        let s8 = span_gateway_translate("FIX", "FIG");
        assert_span_name(&s8, "fig.gateway.translate");

        // Verify field metadata: each span should carry its field(s).
        fn assert_has_field(span: &Span, field_name: &str) {
            let meta = span.metadata().unwrap();
            let found = meta.fields().iter().any(|f| f.name() == field_name);
            assert!(
                found,
                "span '{}' should have field '{}'",
                meta.name(),
                field_name
            );
        }

        assert_has_field(&s1, "channel_id");
        assert_has_field(&s1, "frame_type");
        assert_has_field(&s2, "channel_id");
        assert_has_field(&s3, "channel_id");
        assert_has_field(&s3, "mode");
        assert_has_field(&s4, "channel_id");
        assert_has_field(&s5, "session_id");
        assert_has_field(&s6, "session_id");
        assert_has_field(&s7, "remote");
        assert_has_field(&s8, "from");
        assert_has_field(&s8, "to");
    }

    #[test]
    fn test_span_instrument() {
        use std::sync::Mutex;

        // Use Span::in_scope to wrap a synchronous closure inside a span.
        // Without an active subscriber, spans are disabled, but the closure
        // must still execute and the span object must remain valid.
        let span = span_encode(42, "Response");

        let tracker = Mutex::new(Vec::new());

        span.in_scope(|| {
            let mut t = tracker.lock().unwrap();
            t.push("executed");
        });

        assert_eq!(*tracker.lock().unwrap(), vec!["executed"]);

        // Verify the span's metadata is accessible even after the scope.
        assert_eq!(
            span.metadata().map(|m| m.name()),
            Some("fig.encode")
        );

        // Nested in_scope: verify the closure returns a value through the span.
        let result = span_encode(99, "Heartbeat").in_scope(|| {
            let mut t = tracker.lock().unwrap();
            t.push("nested");
            42u64
        });
        assert_eq!(result, 42);
        assert_eq!(
            *tracker.lock().unwrap(),
            vec!["executed", "nested"]
        );
    }
}
