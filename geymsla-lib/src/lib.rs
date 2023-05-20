use std::collections::HashMap;

pub struct Geymsla {
    pub strings: HashMap<String, String>,
}

impl Geymsla {
    pub fn empty() -> Self {
        Self {
            strings: HashMap::default(),
        }
    }

    pub fn insert_string(&mut self, string: &str) -> std::io::Result<String> {
        println!("Inserting string: {string}");

        Ok(string.to_string())
    }
}
