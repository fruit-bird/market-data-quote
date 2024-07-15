use crate::quote::Quote;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, Copy, Default)]
pub struct Price(pub [u8; 5]);

impl Price {
    fn to_f64(&self) -> f64 {
        let price_str = std::str::from_utf8(&self.0).unwrap_or("0");
        let price = price_str.parse().unwrap_or(0.0);
        price / 100.0
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Quantity(pub [u8; 7]);

impl Quantity {
    fn to_f64(&self) -> f64 {
        let qty_str = std::str::from_utf8(&self.0).unwrap_or("0");
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
        if &data[start_index - 1..start_index + 4] != b"B6034" {
            eprintln!("Invalid start of message, skipping");
            // println!("{:?}", &data[start_index - 1..start_index + 4]);
            return None;
        }
        assert!(data.last() == Some(&255), "Invalid end of message");

        let (head, body, _tail) = unsafe { data.align_to::<Self>() };
        assert!(head.is_empty(), "Data was not aligned");
        // let x = data.iter().find(|&&b| b == 66);
        // println!("66 found? {:?}", x);
        // println!("ASCII {:?}", b"B6034");
        // println!("data {:?}", data.len());
        // println!("tail {:?}", _tail);

        // println!("tail {:?}", _tail.len());
        // println!("{:?}", data);
        Some(body[0])
        // todo!()
    }

    pub fn to_quote(&self, system_time: SystemTime) -> Quote {
        let issue_code = std::str::from_utf8(&self.issue_code)
            .unwrap_or("")
            .to_string();
        let bids = self
            .bids
            .iter()
            .map(|(price, quantity)| (price.to_f64(), quantity.to_f64()))
            .collect();
        let asks = self
            .asks
            .iter()
            .map(|(price, quantity)| (price.to_f64(), quantity.to_f64()))
            .collect();

        Quote {
            packet_time: system_time,
            // accept_time: self.parse_accept_time(), // TODO
            accept_time: system_time,
            issue_code,
            bids,
            asks,
        }
    }

    pub fn parse_accept_time(&self) -> SystemTime {
        let time_str = std::str::from_utf8(&self.quote_accept_time).unwrap_or("000000000");
        let hours: u64 = time_str[0..2].parse().unwrap_or(0);
        let minutes: u64 = time_str[2..4].parse().unwrap_or(0);
        let seconds: u64 = time_str[4..6].parse().unwrap_or(0);
        let micros: u64 = time_str[6..8].parse().unwrap_or(0) * 10000; // Convert hundredths to microseconds

        let now = chrono::Utc::now().naive_utc().date();
        let naive_dt = now
            .and_hms_micro_opt(hours as u32, minutes as u32, seconds as u32, micros as u32)
            .unwrap();

        SystemTime::UNIX_EPOCH
            + Duration::from_secs(naive_dt.and_utc().timestamp() as u64)
            + Duration::from_micros(naive_dt.and_utc().timestamp_subsec_micros() as u64)
    }
}
