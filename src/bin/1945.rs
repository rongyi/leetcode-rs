struct Solution;

impl Solution {
    pub fn get_lucky(s: String, mut k: i32) -> i32 {
        let mut vals: String = s
            .chars()
            .map(|c| (c as i32 - 'a' as i32 + 1).to_string())
            .collect();
        while k > 0 {
            k -= 1;
            let next: String = vals
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .sum::<u32>()
                .to_string();
            vals = next;
        }
        vals.parse().unwrap()
    }
}

fn main() {}
