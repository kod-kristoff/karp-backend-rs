use webapp::startup;

#[tokio::main]
async fn main() {
    if let Err(e) = startup::run().await {
        eprintln!("Server error: {}", e);
    }
}
