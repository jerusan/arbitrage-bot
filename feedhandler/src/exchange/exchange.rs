pub trait Exchange {
    fn connect(&mut self);
   
    fn subscribe(&mut self);
    
     fn receive_message(&mut self);
  }