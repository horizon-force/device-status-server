use tokio::runtime::Runtime;

fn main() {
    Runtime::new()
        .expect("Unable to create tokio runtime")
        .block_on(async {
            server::app::run().await;
        });
}
