use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

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
        let msg = format!("SSET {} {}", key, value);
        self.request(&msg).await
    }

    pub async fn get_string(&self, key: &str) -> Result<String> {
        let msg = format!("SGET {}", key);
        self.request(&msg).await
    }

    async fn request(&self, msg: &str) -> Result<String> {
        let response = self.send(&msg).await?;
        let contents = self.parse_response(&response);

        Ok(contents)
    }

    fn parse_response(&self, resp: &str) -> String {
        let resp_split = resp.split(": ").collect::<Vec<&str>>();
        if resp_split.len() == 0 {
            return String::from("");
        }

        resp_split[1].to_string()
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
