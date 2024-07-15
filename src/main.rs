mod pcap_parser;
mod quote;
mod quote_packet;

use clap::Parser;
use pcap_parser::parse_pcap_file;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(bin_name = "parse-quote")]
pub struct ParseQuote {
    #[arg(name = "FILE", help = "Input PCAP file")]
    input: PathBuf,
    #[arg(short, long, help = "Reorder quotes by accept time")]
    reorder: bool,
}

fn main() -> Result<(), anyhow::Error> {
    let args = ParseQuote::parse();
    let mut quotes = parse_pcap_file(&args.input)?;

    if args.reorder {
        eprintln!("Reordering packets");
        quotes.sort_by_key(|q| q.accept_time);
    }

    // println!("{:?} quotes", quotes.len());
    for quote in quotes.iter() {
        println!("{}", quote);
    }

    Ok(())
}
