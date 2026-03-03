struct Solution;

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        let mut ret = 0;
        for i in 0..=limit {
            for j in 0..=limit {
                let k = n - i - j;
                if k >= 0 && k <= limit {
                    ret += 1;
                }
            }
        }
        ret
    }
}

fn main() {}
