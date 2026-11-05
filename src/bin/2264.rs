struct Solution;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        for i in (0..=9).rev() {
            let cur = i.to_string().repeat(3);
            if num.contains(&cur) {
                return cur;
            }
        }
        "".to_string()
    }
}

fn main() {}
