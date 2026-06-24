//! High-level FIG client over a QUIC bidirectional stream per operation.

use fig_core::codec::{decode_cbor, encode_cbor};
use fig_core::frame::{Frame, FrameDecoder, FrameType};
use fig_core::messages::{
    AllMidsBatch, AllMidsRequest, BestBidOffer, CandleBarBatch, CandleBarRequest, ExecutionReport,
    FillHistoryBatch, FillHistoryRequest, MarkPriceUpdate, MiniTicker, NewOrderSingle,
    OpenOrdersSnapshot, OrderBookSnapshot, PublicTradeEvent,
};
use quinn::Connection;
use thiserror::Error;

use crate::bbo::BboState;
use crate::candles::CandleState;
use crate::dev_auth_token;
use crate::frames::{request_frame, subscribe_frame};
use crate::mark_price::MarkPriceState;
use crate::mids::MidsState;
use crate::order_book::OrderBookState;
use crate::orders::OrdersState;
use crate::trades::TradeTape;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("frame encode: {0}")]
    FrameEncode(#[from] fig_core::FrameError),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("quinn: {0}")]
    Quinn(#[from] quinn::ConnectionError),
    #[error("quinn read: {0}")]
    QuinnRead(#[from] quinn::ReadError),
    #[error("quinn write: {0}")]
    QuinnWrite(#[from] quinn::WriteError),
    #[error("stream closed")]
    StreamClosed(#[from] quinn::ClosedStream),
    #[error("codec: {0}")]
    Codec(String),
    #[error("no response")]
    NoResponse,
}

pub struct FigSdkClient<'a> {
    conn: &'a Connection,
}

impl<'a> FigSdkClient<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub async fn send_and_read(&self, frame: Frame) -> Result<Vec<Frame>, ClientError> {
        let (mut send, mut recv) = self.conn.open_bi().await?;
        send.write_all(&frame.encode()?)
            .await
            .map_err(ClientError::QuinnWrite)?;
        send.finish()?;
        let mut decoder = FrameDecoder::new();
        let mut buf = vec![0u8; 8192];
        let mut frames = Vec::new();
        while let Some(n) = recv.read(&mut buf).await? {
            decoder.feed(&buf[..n]);
            while let Some(r) = decoder.decode_next() {
                frames.push(r?);
            }
        }
        Ok(frames)
    }

    pub async fn subscribe_order_book(
        &self,
        symbol: &str,
        channel_id: u16,
    ) -> Result<(OrderBookState, Vec<Frame>), ClientError> {
        let path = format!("marketdata/{symbol}/book");
        let frame = subscribe_frame(channel_id, 1, &path, Some(&path), None)?;
        let frames = self.send_and_read(frame).await?;
        let mut state = OrderBookState::default();
        for f in &frames {
            if let Ok(snap) = decode_cbor::<OrderBookSnapshot>(&f.payload) {
                let _ = state.apply_snapshot(&snap);
            } else if let Ok(snap) =
                decode_cbor::<fig_core::messages::MarketDataSnapshot>(&f.payload)
            {
                let _ = state.apply_market_data_snapshot(&snap);
            } else if let Ok(delta) = decode_cbor::<fig_core::messages::OrderBookDelta>(&f.payload)
            {
                let _ = state.apply_delta(&delta);
            }
        }
        Ok((state, frames))
    }

    pub async fn subscribe_candles(
        &self,
        symbol: &str,
        interval: &str,
        channel_id: u16,
    ) -> Result<(CandleState, Vec<Frame>), ClientError> {
        let path = format!("marketdata/{symbol}/candles/{interval}");
        let frame = subscribe_frame(channel_id, 1, &path, Some(&path), None)?;
        let frames = self.send_and_read(frame).await?;
        let mut state = CandleState::default();
        for f in &frames {
            if let Ok(ev) = decode_cbor::<fig_core::messages::CandleBarEvent>(&f.payload) {
                state.apply_event(&ev);
            }
        }
        Ok((state, frames))
    }

    pub async fn request_candles(
        &self,
        req: CandleBarRequest,
        channel_id: u16,
    ) -> Result<CandleBarBatch, ClientError> {
        let symbol = req.symbol.clone();
        let interval = req.interval.clone();
        let path = format!("marketdata/{symbol}/candles/{interval}");
        let payload = encode_cbor(&req).map_err(|e| ClientError::Codec(e.to_string()))?;
        let frame = request_frame(channel_id, 1, 0x01, &path, "GET", Some(payload), None)?;
        let frames = self.send_and_read(frame).await?;
        for f in frames {
            if f.frame_type == FrameType::Response {
                return decode_cbor(&f.payload).map_err(|e| ClientError::Codec(e.to_string()));
            }
        }
        Err(ClientError::NoResponse)
    }

    pub async fn request_fills(
        &self,
        account: &str,
        req: FillHistoryRequest,
        channel_id: u16,
    ) -> Result<FillHistoryBatch, ClientError> {
        let path = format!("trading/accounts/{account}/fills");
        let token = dev_auth_token(account);
        let payload = encode_cbor(&req).map_err(|e| ClientError::Codec(e.to_string()))?;
        let frame = request_frame(
            channel_id,
            1,
            0x01,
            &path,
            "GET",
            Some(payload),
            Some(&token),
        )?;
        let frames = self.send_and_read(frame).await?;
        for f in frames {
            if f.frame_type == FrameType::Response {
                return decode_cbor(&f.payload).map_err(|e| ClientError::Codec(e.to_string()));
            }
        }
        Err(ClientError::NoResponse)
    }

    pub async fn post_order(
        &self,
        account: &str,
        order: &NewOrderSingle,
        channel_id: u16,
    ) -> Result<Vec<Frame>, ClientError> {
        let path = format!("trading/accounts/{account}/orders");
        let token = dev_auth_token(account);
        let payload = encode_cbor(order).map_err(|e| ClientError::Codec(e.to_string()))?;
        let frame = request_frame(
            channel_id,
            1,
            0x01,
            &path,
            "POST",
            Some(payload),
            Some(&token),
        )?;
        self.send_and_read(frame).await
    }

    pub async fn request_open_orders(
        &self,
        account: &str,
        channel_id: u16,
    ) -> Result<OpenOrdersSnapshot, ClientError> {
        let path = format!("trading/accounts/{account}/orders/open");
        let token = dev_auth_token(account);
        let req = fig_core::messages::OpenOrdersRequest {
            account: account.to_string(),
            symbol: None,
        };
        let payload = encode_cbor(&req).map_err(|e| ClientError::Codec(e.to_string()))?;
        let frame = request_frame(
            channel_id,
            1,
            0x01,
            &path,
            "GET",
            Some(payload),
            Some(&token),
        )?;
        let frames = self.send_and_read(frame).await?;
        for f in frames {
            if f.frame_type == FrameType::Response {
                return decode_cbor(&f.payload).map_err(|e| ClientError::Codec(e.to_string()));
            }
        }
        Err(ClientError::NoResponse)
    }

    pub async fn request_all_mids(
        &self,
        channel_id: u16,
    ) -> Result<(MidsState, AllMidsBatch), ClientError> {
        let path = "marketdata/ticker/all";
        let payload =
            encode_cbor(&AllMidsRequest {}).map_err(|e| ClientError::Codec(e.to_string()))?;
        let frame = request_frame(channel_id, 1, 0x01, path, "GET", Some(payload), None)?;
        let frames = self.send_and_read(frame).await?;
        for f in frames {
            if f.frame_type == FrameType::Response {
                let batch: AllMidsBatch =
                    decode_cbor(&f.payload).map_err(|e| ClientError::Codec(e.to_string()))?;
                let mut state = MidsState::default();
                state.apply_batch(&batch);
                return Ok((state, batch));
            }
        }
        Err(ClientError::NoResponse)
    }

    pub async fn subscribe_bbo(
        &self,
        symbol: &str,
        channel_id: u16,
    ) -> Result<(BboState, Vec<Frame>), ClientError> {
        let path = format!("marketdata/{symbol}/bbo");
        let frame = subscribe_frame(channel_id, 1, &path, Some(&path), None)?;
        let frames = self.send_and_read(frame).await?;
        let mut state = BboState::default();
        for f in &frames {
            if let Ok(bbo) = decode_cbor::<BestBidOffer>(&f.payload) {
                state.apply(&bbo);
            }
        }
        Ok((state, frames))
    }

    pub async fn subscribe_trades(
        &self,
        symbol: &str,
        channel_id: u16,
    ) -> Result<(TradeTape, Vec<Frame>), ClientError> {
        let path = format!("marketdata/{symbol}/trades");
        let frame = subscribe_frame(channel_id, 1, &path, Some(&path), None)?;
        let frames = self.send_and_read(frame).await?;
        let mut tape = TradeTape::default();
        for f in &frames {
            if let Ok(ev) = decode_cbor::<PublicTradeEvent>(&f.payload) {
                tape.push_event(&ev);
            }
        }
        Ok((tape, frames))
    }

    pub async fn subscribe_mark_price(
        &self,
        symbol: &str,
        channel_id: u16,
    ) -> Result<(MarkPriceState, Vec<Frame>), ClientError> {
        let path = format!("marketdata/{symbol}/mark");
        let frame = subscribe_frame(channel_id, 1, &path, Some(&path), None)?;
        let frames = self.send_and_read(frame).await?;
        let mut state = MarkPriceState::default();
        for f in &frames {
            if let Ok(upd) = decode_cbor::<MarkPriceUpdate>(&f.payload) {
                state.apply(&upd);
            }
        }
        Ok((state, frames))
    }

    pub async fn subscribe_executions(
        &self,
        account: &str,
        channel_id: u16,
    ) -> Result<(OrdersState, Vec<Frame>), ClientError> {
        let path = format!("trading/accounts/{account}/executions");
        let token = dev_auth_token(account);
        let frame = subscribe_frame(channel_id, 1, &path, Some(&path), Some(&token))?;
        let frames = self.send_and_read(frame).await?;
        let mut state = OrdersState::default();
        state.account = account.to_string();
        for f in &frames {
            if let Ok(report) = decode_cbor::<ExecutionReport>(&f.payload) {
                state.apply_execution_report(&report);
            } else if let Ok(snap) = decode_cbor::<OpenOrdersSnapshot>(&f.payload) {
                state.apply_open_orders_snapshot(&snap);
            }
        }
        Ok((state, frames))
    }

    /// Apply stream payloads from a frame batch into the appropriate merge states.
    pub fn apply_stream_frames(
        mids: &mut MidsState,
        bbo: &mut BboState,
        tape: &mut TradeTape,
        marks: &mut MarkPriceState,
        orders: &mut OrdersState,
        frames: &[Frame],
    ) {
        for f in frames {
            if let Ok(t) = decode_cbor::<MiniTicker>(&f.payload) {
                mids.apply_ticker(&t);
            } else if let Ok(b) = decode_cbor::<AllMidsBatch>(&f.payload) {
                mids.apply_batch(&b);
            } else if let Ok(bbo_upd) = decode_cbor::<BestBidOffer>(&f.payload) {
                bbo.apply(&bbo_upd);
            } else if let Ok(ev) = decode_cbor::<PublicTradeEvent>(&f.payload) {
                tape.push_event(&ev);
            } else if let Ok(m) = decode_cbor::<MarkPriceUpdate>(&f.payload) {
                marks.apply(&m);
            } else if let Ok(r) = decode_cbor::<ExecutionReport>(&f.payload) {
                orders.apply_execution_report(&r);
            } else if let Ok(snap) = decode_cbor::<OpenOrdersSnapshot>(&f.payload) {
                orders.apply_open_orders_snapshot(&snap);
            }
        }
    }
}
