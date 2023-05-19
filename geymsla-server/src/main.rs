use std::collections::{HashMap, HashSet};
use tokio::net::TcpListener;

const DEFAULT_GEYMSLA_PORT: usize = 9876;

pub struct Geymsla {
    pub strings: HashMap<String, String>,
    pub lists: HashMap<String, Vec<String>>,
    pub sets: HashMap<String, HashSet<String>>,
    pub hashes: HashMap<String, HashMap<String, String>>,
}

impl Geymsla {
    pub fn empty() -> Self {
        Self {
            strings: HashMap::default(),
            lists: HashMap::default(),
            sets: HashMap::default(),
            hashes: HashMap::default(),
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut geymsla = Geymsla::empty();
    let addr = format!("0.0.0.0:{}", DEFAULT_GEYMSLA_PORT);
    let listener = TcpListener::bind(&addr).await?;

    println!("Started Geymsla server");
    loop {
        let (socket, _) = listener.accept().await?;
        println!("Accepted new client: {}", socket.peer_addr()?);
    }
}
