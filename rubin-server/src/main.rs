use rubin_server::start;

const DEFAULT_PORT: usize = 9876;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    start("127.0.0.1", DEFAULT_PORT).await
}
