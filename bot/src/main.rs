use feedhandler::FeedHandler;

fn main() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        FeedHandler::get_feed().await;
    });
}
