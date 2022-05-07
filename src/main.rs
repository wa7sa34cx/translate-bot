//! Main module

use translate_bot::run;

#[tokio::main]
async fn main() {
    run().await.unwrap();
}
