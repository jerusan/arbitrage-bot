fn main() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        feedhandler::FeedHandler::get_feed().await;
    });
}
