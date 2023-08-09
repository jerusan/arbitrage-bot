use super::exchange::Exchange;
use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use tungstenite::{connect, stream::MaybeTlsStream, Message as TMessage, WebSocket};

pub struct Gemini {
    pub socket: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
}

impl Gemini {
    pub fn default() -> Self {
        Gemini { socket: None }
    }
}

impl Exchange for Gemini {
    fn connect(&mut self) {
        println!("Starting Gemini feed handler");

        let url = url::Url::parse("wss://api.gemini.com/v2/marketdata").unwrap();

        let (ws_socket, _response) = connect(url).expect("Failed to connect");
        println!("WebSocket handshake has been successfully completed");

        self.socket = Some(ws_socket);
    }

    fn subscribe(&mut self) {
        // Make subscription API requests
        println!("Sending 'ticker' channel subscription request");

        self.socket
            .as_mut()
            .unwrap()
            .write_message(TMessage::Text(
                r#"{"type": "subscribe","subscriptions":[{"name":"l2","symbols":["BTCUSD"]}]}"#
                    .into(),
            ))
            .unwrap();

        println!("Subscription request sent!");
    }

    fn receive_message(&mut self) {
        while let Ok(msg) = self.socket.as_mut().unwrap().read_message() {
            // Deserialize the content into a JSON object
            match msg {
                TMessage::Text(content) => match serde_json::from_str::<TickerGemini>(&content) {
                    Ok(json_object) => {
                        println!(
                            "Gemini price: {}\n",
                            json_object.changes.first().unwrap().get(1).unwrap()
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerGemini {
    pub changes: Vec<Vec<String>>,
    pub symbol: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
