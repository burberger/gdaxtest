use std::thread;
use std::sync::mpsc::{Receiver, sync_channel};
use websocket::{ClientBuilder, Message};
use websocket::message::OwnedMessage;
use serde_json;
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct GDAXMessage {
    #[serde(rename = "type")]
    pub type_name: String,
    pub trade_id: i64,
    pub sequence: i64,
    pub maker_order_id: String,
    pub taker_order_id: String,
    pub time: String,
    pub product_id: String,
    pub size: String,
    pub price: String,
    pub side: String
}

pub fn start_receiver_thread(url: &str, products: Vec<&str>) -> Receiver<GDAXMessage> {
    // Construct a websocket client to connect to the exchange server.
    let mut client = match ClientBuilder::new(url) {
        Ok(mut client_builder) => {
            match client_builder.connect(None) {
                Ok(client) => client,
                Err(err) => panic!("Failed to connect: {:?}", err),
            }
        },
        Err(err) => panic!("Failed to parse url: {:?}", err),
    };

    // Issue the subscribe request to the exchange.
    let msg = json!({"type": "subscribe", "product_ids": products});
    client.send_message(&Message::text(msg.to_string()))
            .expect("Failed to send message");

    /*
     * Push the receiver off into a seperate thread, queueing each message out
     * to the main thread.
     */
    let (tx, rx) = sync_channel(500);
    thread::spawn(move || {
        for message in client.incoming_messages() {
            match message {
                Ok(message) => {
                    match parse_message(message) {
                        Some(value) => tx.send(value).unwrap(),
                        None => (),
                    };
                },
                Err(err) => panic!("Failed to process message: {:?}", err),
            };
        }
    });

    rx
}

fn parse_message(message: OwnedMessage) -> Option<GDAXMessage> {
    match message {
        OwnedMessage::Text(data) => {
            match serde_json::from_str(data.as_str()) {
                Ok(message) => Some(message),
                Err(_) => None,
            }
        },
        _ => None,
    }
}
