use std::collections::HashMap;

struct Codec {
    counter: u64,
    url_map: HashMap<String, String>,
    base_url: String,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {
            counter: 0,
            url_map: HashMap::new(),
            base_url: "http://tinyurl.com/".to_string(),
        }
    }

    // Encodes a URL to a shortened URL.
    fn encode(&mut self, long_url: String) -> String {
        let short_url = self.base_url.clone() + &self.counter.to_string();
        self.url_map.insert(short_url.clone(), long_url);
        self.counter += 1;

        short_url
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, short_url: String) -> String {
        self.url_map.get(&short_url).unwrap().clone()
    }
}

fn main() {}
