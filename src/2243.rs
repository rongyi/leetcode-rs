struct Solution;

impl Solution {
    pub fn digit_sum(mut s: String, k: i32) -> String {
        let k = k as usize;
        while s.len() > k {
            let mut new_s = String::new();
            for chunk in s.chars().collect::<Vec<char>>().chunks(k) {
                let sum: u32 = chunk.iter().map(|c| c.to_digit(10).unwrap()).sum();
                new_s.push_str(&sum.to_string());
            }
            s = new_s;
        }

        s
    }
}
fn main() {}
