use feedhandler::exchange::{
    coinbase::Coinbase,
    exchange::Exchange,
    gemini::Gemini,
};
use tokio::runtime::Runtime;


fn main() {
    println!("Starting feed handler");

    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
    
        tokio::spawn(async {
            let mut gemini = Gemini::connect().await;
            Gemini::subscribe(gemini.as_mut()).await;
            Gemini::receive_message(gemini.as_mut()).await;
        });
        tokio::spawn(async {
            let mut coinbase = Coinbase::connect().await;
            Coinbase::subscribe(coinbase.as_mut()).await;
            Coinbase::receive_message(coinbase.as_mut()).await;
        });

       
    });

    loop {}
   
}
