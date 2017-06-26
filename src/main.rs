extern crate websocket;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod gdax_client;

use std::time::{SystemTime, Duration};
use std::thread;

fn main() {
    // Start a mark time thread, spins and reports what time it is.
    thread::spawn(|| {
        loop {
            println!("{:?}", SystemTime::now());
            thread::sleep(Duration::from_secs(300));
        }
    });

    let rx_chan = gdax_client::start_receiver_thread("wss://ws-feed.gdax.com",
                                                     vec![String::from("BTC-USD")]);

    let mut price_accum = 0.0;
    let mut msg_count = 0;
    let mut start_time = SystemTime::now();
    for data in rx_chan {
        if data.type_name == "match" {
            price_accum += data.price.parse().unwrap();
            msg_count += 1;
        }

        if start_time.elapsed().unwrap().as_secs() > 30 {
            if msg_count > 0 {
                let avg = price_accum / (msg_count as f64);
                println!("Average price: {}", avg);
            }
            start_time = SystemTime::now();
        }
    }
}
