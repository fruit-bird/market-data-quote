use chrono::DateTime;
use rayon::prelude::*;
use std::{fmt::Display, time::SystemTime};

#[allow(unused)]
#[derive(Debug)]
pub struct Quote {
    pub packet_time: SystemTime,
    pub accept_time: SystemTime,
    pub issue_code: String,
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
}

impl Display for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let packet_time = format_time(self.packet_time);
        let accept_time = format_time(self.accept_time);
        let bids = format_list(&self.bids);
        let asks = format_list(&self.asks);
        write!(
            f,
            "{}\t{}\t{}\t{}\t{}",
            packet_time, accept_time, self.issue_code, bids, asks
        )
    }
}

fn format_time(time: SystemTime) -> String {
    let duration = time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let datetime =
        DateTime::from_timestamp(duration.as_secs() as i64, duration.subsec_nanos()).unwrap();
    datetime.format("%H:%M:%S%.6f").to_string()
}

fn format_list(list: &[(f64, f64)]) -> String {
    list.par_iter()
        .rev()
        .map(|(price, qty)| format!("{: >4.2}@{: <12.2}", price, qty))
        .collect::<Vec<_>>()
        .join(" ")
}
