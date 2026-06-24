//! Exchange-sim multi-codec dispatch (CBOR / SBE / Protobuf via `fig-core::codec`).

use fig_core::codec;
use fig_core::frame::Frame;
use fig_core::messages::NewOrderSingle;

/// Decode a trading request payload using the frame `ContentType` extension.
pub fn decode_new_order(frame: &Frame) -> Result<NewOrderSingle, fig_core::error::FrameError> {
    codec::decode_new_order_single_frame(frame)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fig_core::ext::{Extension, ExtensionTag};
    use fig_core::frame::{Frame, FrameType};
    use fig_core::messages::{OrderType, Price, Quantity, Side, TimeInForce};

    fn sample_order() -> NewOrderSingle {
        NewOrderSingle {
            cl_ord_id: "MC-1".to_string(),
            side: Side::Buy,
            order_qty: Quantity(1.0),
            price: Some(Price(10.0)),
            stop_price: None,
            symbol: "AAPL".to_string(),
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::Day,
            expire_time: None,
            account: Some("TEST".to_string()),
            strategy_id: None,
            security_id: None,
            id_source: None,
            security_exchange: None,
            post_only: None,
            reduce_only: None,
        }
    }

    #[test]
    fn cbor_and_sbe_new_order_round_trip() {
        let order = sample_order();
        let cbor_payload = codec::encode_cbor(&order).unwrap();
        let sbe_payload =
            codec::encode_new_order_single_payload(&order, "application/fig+sbe").unwrap();

        let cbor_frame = Frame::new(FrameType::Request, 1)
            .with_extension(Extension::text(
                ExtensionTag::ContentType,
                "application/cbor",
            ))
            .with_payload(cbor_payload);
        let sbe_frame = Frame::new(FrameType::Request, 1)
            .with_extension(Extension::text(
                ExtensionTag::ContentType,
                "application/fig+sbe",
            ))
            .with_payload(sbe_payload);

        assert_eq!(decode_new_order(&cbor_frame).unwrap().cl_ord_id, "MC-1");
        assert_eq!(decode_new_order(&sbe_frame).unwrap().cl_ord_id, "MC-1");
    }
}
