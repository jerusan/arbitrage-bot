use feedhandler::exchange::{
    coinbase::Coinbase,
    exchange::Exchange,
    gemini::Gemini,
};
use tokio::runtime::Runtime;


fn main() {
    println!("Starting Bot");

    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
    
        tokio::spawn(async {
            let mut gemini = Gemini::default();
            gemini.connect();
            gemini.subscribe();
            gemini.receive_message();
          
        });
        tokio::spawn(async {
            let mut coinbase = Coinbase::default();
            coinbase.connect();
            coinbase.subscribe();
            coinbase.receive_message();
        });

       
    });

    loop {}
   
}
