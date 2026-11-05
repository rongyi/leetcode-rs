struct Solution;

impl Solution {
    pub fn base_neg2(mut n: i32) -> String {
        if n == 0 {
            return "0".to_string();
        }
        let mut ret = String::new();
        while n != 0 {
            let mut rem = n % -2;
            n /= -2;
            if rem < 0 {
                rem += 2;
                // cancel the rem +2 action
                n += 1;
            }
            ret.push_str(&rem.to_string());
        }

        ret.chars().rev().collect()
    }
}

fn main() {}
