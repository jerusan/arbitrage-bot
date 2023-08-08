use async_trait::async_trait;

#[async_trait]
pub trait Exchange<T> {
    async fn connect() -> Box<T>;
    // , channels: &[String]
    async fn subscribe(&mut self);
    
    async  fn receive_message(&mut self);
  }