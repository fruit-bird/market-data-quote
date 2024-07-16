# Market Data Quote Parser
A simple parser for market data quotes. Data is received as packets from a UDP stream. In this proof of concept the packets are read from a `.pcap` file. It reads the file of market data quotes, and outputs details as shown below. ([Full output](./output.md))

```
00:00:00.538991	10:00:00.000000		0.00@9010250.00   0.71@2010300.00   0.00@4085110.00   0.00@0.00         0.00@0.00        	0.00@8014000.00   0.63@2013950.00   101.00@0.00         101.50@0.00         102.00@90.00
```

By leveraging the [rayon](https://crates.io/crates/rayon) crate, we can parallelize most iterations, and speed up the parsing process. Some of the parsing is still done sequentially because we are parsing an offline UDP stream. In a real-world scenario with a live stream, we could completely parallelize the parsing of the packets.

Output is formatted like a `.tsv`, so that it can be easily used elsewhere (spreadsheets...). Create the output file as follows:
```bash
parse-quote <FILE> > output.tsv
```

## Benchmark
The example file contains 23498 entries

- With the I/O overhead of writing the output to the **terminal**, the parser takes about **1.7 seconds** to parse the file.
`1.73s user 7.36s system 494% cpu 1.838 total`
- With the I/O overhead of writing the output to a **file**, the parser takes about **1.25 seconds** to parse the file.
`1.26s user 5.23s system 440% cpu 1.472 total`

## What is This?
This is actually part of an application for Tsuru Capital. Got to learn about UDP parsing, and a pretty nifty technique of quickly mapping bytes to a struct. I'm pretty happy with the results, and I think it's a pretty good example of my coding style :)
