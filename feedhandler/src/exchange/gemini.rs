// {"type": "subscribe","subscriptions":[{"name":"l2","symbols":["BTCUSD","ETHUSD","ETHBTC"]}]}

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as TMessage};
pub type WebSocketStream =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

use serde::{Deserialize, Serialize};

use super::exchange::Exchange;

pub struct Gemini {
    pub write: SplitSink<WebSocketStream, TMessage>,
    pub read: SplitStream<WebSocketStream>,
}
use async_trait::async_trait;

#[async_trait]
impl Exchange<Gemini> for Gemini {
    async fn connect() -> Box<Gemini> {
        println!("Starting Gemini feed handler");

        let url = url::Url::parse("wss://api.gemini.com/v2/marketdata").unwrap();

        let (ws_stream, _response) = connect_async(url).await.expect("Failed to connect");
        println!("WebSocket handshake has been successfully completed");

        let (write, read) = ws_stream.split();

        Box::new(Gemini { write, read })
    }
    // , channels: &[String]
     async fn subscribe(&mut self) {
        // Make subscription API requests
        println!("Sending 'ticker' channel subscription request");

        self.write
            .send(TMessage::Text(
                r#"{"type": "subscribe","subscriptions":[{"name":"l2","symbols":["BTCUSD"]}]}"#
                .into(),
            ))
            .await
            .unwrap();

        println!("Subscription request sent!");
       
    }

     async fn receive_message(&mut self) {
        while let Some(msg) = &mut self.read.next().await {
            // Unwrap result
            match msg {
                Ok(m) => {
                    if let TMessage::Text(content) = m {
                        match serde_json::from_str::<TickerGemini>(&content) {
                            Ok(json_object) => {
                                tokio::io::stdout()
                                    .write(format!("Gemini price: {}\n", json_object.changes.first().unwrap().get(1).unwrap()).as_bytes())
                                    .await
                                    .unwrap();
                            }
                            Err(err) => {
                                println!("Error deserializing JSON object: {}", err);
                            }
                        }
                    } else {
                        println!("Received a non-TEXT message");
                    }
                }
                Err(e) => {
                    continue;
                }
            };
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