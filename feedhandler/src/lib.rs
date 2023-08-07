use serde::{Deserialize, Serialize};

use futures_util::{SinkExt, StreamExt};
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as TMessage};
pub struct FeedHandler;

impl FeedHandler {
    pub async fn get_feed() {
        println!("Starting feed handler");

        let url = url::Url::parse("wss://ws-feed-public.sandbox.exchange.coinbase.com").unwrap();

        let (ws_stream, _response) = connect_async(url).await.expect("Failed to connect");
        println!("WebSocket handshake has been successfully completed");

        let (mut write, read) = ws_stream.split();

        println!("Sending 'ticker' channel subscription request");

        write
            .send(TMessage::Text(
                r#"{
                    "type": "subscribe",
            "product_ids": [
                "BTC-USD"
            ],
            "channels": ["ticker"]
                }"#
                .into(),
            ))
            .await
            .unwrap();

        println!("Subscription request sent!");

        let read_future = read.for_each(|message| async {
            match message.unwrap().into() {
                TMessage::Text(content) => match serde_json::from_str::<Ticker>(&content) {
                    Ok(json_object) => {
                        // println!("Deserialized JSON object: {:?}", json_object.best_ask_size);
                        tokio::io::stdout()
                            .write(format!("{}\n", json_object.best_ask_size).as_bytes())
                            .await
                            .unwrap();
                    }
                    Err(err) => {
                        println!("Error deserializing JSON object: {}", err);
                    }
                },
                _ => {
                    println!("Received a non-TEXT message");
                }
            }
        });

        read_future.await;
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
