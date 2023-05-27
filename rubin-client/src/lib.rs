use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use rubin_lib::net::parser::{create_request, parse_response, Operation};

use std::io::Result;

pub struct RubinClient {
    pub address: String,
}

impl RubinClient {
    pub fn new(addr: &str, port: usize) -> Self {
        let address = format!("{}:{}", addr, port);
        Self { address }
    }

    pub async fn insert_string(&self, key: &str, value: &str) -> Result<String> {
        let msg = create_request(
            Operation::StringSet,
            vec![key.to_string(), value.to_string()],
        );
        self.request(&msg).await
    }

    pub async fn get_string(&self, key: &str) -> Result<String> {
        let msg = create_request(Operation::StringGet, vec![key.to_string()]);
        self.request(&msg).await
    }

    async fn request(&self, msg: &str) -> Result<String> {
        let response = self.send(&msg).await?;
        let contents = parse_response(&response);

        Ok(contents)
    }

    async fn send(&self, msg: &str) -> Result<String> {
        let mut client = TcpStream::connect(&self.address).await?;
        client.write_all(msg.as_bytes()).await?;

        let mut buffer = [0; 4096];
        let n_bytes = client.read(&mut buffer).await?;
        if n_bytes == 0 {
            return Ok(String::from(""));
        }

        let response = String::from_utf8_lossy(&mut buffer[..n_bytes]);

        Ok(response.to_string())
    }
}
