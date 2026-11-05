struct Solution;

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let s = s.as_bytes();
        let sz = s.len();
        // at least be 1
        let one_count = s.iter().filter(|x| **x == b'1').count();
        let left_one_count = one_count - 1;
        let zero_count = sz - one_count;

        format!(
            "{}{}{}",
            "1".repeat(left_one_count),
            "0".repeat(zero_count),
            "1"
        )
    }
}

fn main() {}
