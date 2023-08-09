use super::exchange::Exchange;
use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use tungstenite::{connect, stream::MaybeTlsStream, Message as TMessage, WebSocket};

pub struct Coinbase {
    pub socket: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
}
impl Coinbase {
    pub fn default() -> Self {
        Coinbase { socket: None }
    }
}

impl Exchange for Coinbase {
    fn connect(&mut self) {
        println!("Starting Coinbase feed handler");

        let url = url::Url::parse("wss://ws-feed-public.sandbox.exchange.coinbase.com").unwrap();

        let (ws_socket, _response) = connect(url).expect("Failed to connect");
        println!("Coinbase WebSocket handshake has been successfully completed");

        self.socket = Some(ws_socket);
    }
    
    fn subscribe(&mut self) {
        // Make subscription API requests
        println!("Sending 'ticker' channel subscription request");

        self.socket
            .as_mut()
            .unwrap()
            .write_message(TMessage::Text(
                r#"{
                "type": "subscribe",
        "product_ids": [
            "BTC-USD"
        ],
        "channels": ["ticker"]
            }"#
                .into(),
            ))
            .unwrap();

        println!("Subscription request sent!");
    }

    fn receive_message(&mut self) {
        while let Ok(msg) = self.socket.as_mut().unwrap().read_message() {
         
            match msg {
                TMessage::Text(content) => match serde_json::from_str::<Ticker>(&content) {
                    Ok(json_object) => {
                        println!(
                            "Coinbase price:{}, time: {}\n",
                            json_object.price, json_object.time
                        );
                    }
                    Err(err) => {
                        println!("Error deserializing JSON object: {}", err);
                    }
                },
                _ => {
                    println!("Received a non-TEXT message");
                }
            }
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sequence: i64,
    #[serde(rename = "product_id")]
    pub product_id: String,
    pub price: String,
    #[serde(rename = "open_24h")]
    pub open_24h: String,
    #[serde(rename = "volume_24h")]
    pub volume_24h: String,
    #[serde(rename = "low_24h")]
    pub low_24h: String,
    #[serde(rename = "high_24h")]
    pub high_24h: String,
    #[serde(rename = "volume_30d")]
    pub volume_30d: String,
    #[serde(rename = "best_bid")]
    pub best_bid: String,
    #[serde(rename = "best_bid_size")]
    pub best_bid_size: String,
    #[serde(rename = "best_ask")]
    pub best_ask: String,
    #[serde(rename = "best_ask_size")]
    pub best_ask_size: String,
    pub side: String,
    pub time: String,
    #[serde(rename = "trade_id")]
    pub trade_id: i64,
    #[serde(rename = "last_size")]
    pub last_size: String,
}
