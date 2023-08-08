use crate::exchange::Exchange;

pub struct FeedClient<T: Exchange> {
  exchange: T,
  // Other fields like urls, config etc
}

impl<T: Exchange> FeedClient<T> {

  pub async fn connect(&mut self) -> Result<()> {
    self.exchange.connect().await?;
    self.exchange.subscribe(&self.channels)?;
    // Manage reconnect loop
  }

  pub async fn recv_message(&mut self) -> Result<ExchangeMessage> {
    let msg = self.exchange.receive_message().await?;
    // wrap in enum with exchange ID 
    Ok(msg) 
  }

}