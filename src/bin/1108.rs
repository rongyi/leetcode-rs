struct Solution;

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let parts: Vec<&str> = address.split('.').collect();
        parts.join("[.]")
    }
}

fn main() {}
