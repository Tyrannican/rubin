use rubin_client::RubinClient;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let client = RubinClient::new("127.0.0.1", 9876);
    client.insert_string("user:2000", "rebecca").await?;
    sleep(Duration::from_millis(2000)).await;
    let response = client.get_string("user:2000").await?;
    println!("Received: {}", response);
    Ok(())
}
