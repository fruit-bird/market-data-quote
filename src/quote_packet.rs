use crate::quote::Quote;
use chrono::Utc;
use rayon::prelude::*;
use std::{
    str,
    time::{Duration, SystemTime},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Price(pub [u8; 5]);

impl Price {
    fn to_f64(&self) -> f64 {
        let price_str = str::from_utf8(&self.0).unwrap_or("0");
        let price = price_str.parse().unwrap_or(0.0);
        price / 100.0
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Quantity(pub [u8; 7]);

impl Quantity {
    fn to_f64(&self) -> f64 {
        let qty_str = str::from_utf8(&self.0).unwrap_or("0");
        qty_str.parse().unwrap_or(0.0)
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct QuotePacket {
    pub data_type: [u8; 2],
    pub information_type: [u8; 2],
    pub market_type: u8,
    pub issue_code: [u8; 12],
    pub sequence_number: [u8; 3],
    pub market_status: [u8; 2],
    pub total_bid_quote_volume: [u8; 7],
    pub bids: [(Price, Quantity); 5],
    pub total_ask_quote_volume: [u8; 7],
    pub asks: [(Price, Quantity); 5],
    pub best_bid_quote_counts: [u8; 25],
    pub best_ask_quote_counts: [u8; 25],
    pub quote_accept_time: [u8; 8],
    pub end_of_message: u8,
}

impl QuotePacket {
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 214 {
            return None;
        }

        let start_index = data.len() - 214;
        if &data[start_index - 1..start_index + 4] != b"B6034" || data.last() != Some(&255) {
            // eprintln!("Invalid message, skipping");
            return None;
        }

        let (head, body, _tail) = unsafe { data.align_to::<Self>() };
        if !head.is_empty() {
            return None;
        }

        Some(body[0])
    }

    pub fn to_quote(&self, system_time: SystemTime) -> Quote {
        let issue_code = str::from_utf8(&self.issue_code).unwrap_or("").to_string();
        let bids = self
            .bids
            .par_iter()
            .map(|(price, quantity)| (price.to_f64(), quantity.to_f64()))
            .collect();
        let asks = self
            .asks
            .par_iter()
            .map(|(price, quantity)| (price.to_f64(), quantity.to_f64()))
            .collect();

        Quote {
            packet_time: system_time,
            accept_time: self.parse_accept_time(),
            // accept_time: system_time,
            issue_code,
            bids,
            asks,
        }
    }

    pub fn parse_accept_time(&self) -> SystemTime {
        let time_str = str::from_utf8(&self.quote_accept_time).unwrap_or("000000000");

        let hour = time_str[0..2].parse::<u16>().unwrap_or_default();
        let min = time_str[2..4].parse::<u16>().unwrap_or_default();
        let sec = time_str[4..6].parse::<u16>().unwrap_or_default();
        let micro = time_str[6..8].parse::<u16>().unwrap_or_default() as u32 * 10_000;

        let now = Utc::now().to_utc().date_naive();
        let naive_dt = now
            .and_hms_micro_opt(hour as _, min as _, sec as _, micro)
            .unwrap_or_default();

        SystemTime::UNIX_EPOCH
            + Duration::from_secs(naive_dt.and_utc().timestamp() as u64)
            + Duration::from_micros(naive_dt.and_utc().timestamp_subsec_micros() as u64)
    }
}
