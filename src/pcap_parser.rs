use crate::{quote::Quote, quote_packet::QuotePacket};
use pcap::Capture;
use std::{
    path::Path,
    time::{Duration, SystemTime},
};

pub fn parse_pcap_file(path: &Path) -> Result<Vec<Quote>, anyhow::Error> {
    let mut cap = Capture::from_file(path)?;
    let mut quotes = Vec::new();

    while let Ok(packet) = cap.next_packet() {
        // eprintln!("Packet: {:?} {:?}", packet.header.ts.tv_usec, packet.data.len());
        if let Some(quote_packet) = QuotePacket::parse(&packet.data[..]) {
            let packet_time = SystemTime::UNIX_EPOCH
                + Duration::from_micros(
                    packet.header.ts.tv_sec as u64 * 1_000_000 + packet.header.ts.tv_usec as u64,
                );

            quotes.push(quote_packet.to_quote(packet_time));
        }
    }

    Ok(quotes)
}
