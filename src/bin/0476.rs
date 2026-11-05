struct Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut mask = 1;
        while mask < num {
            mask <<= 1;
            mask += 1;
        }
        num ^ mask
    }
}

fn main() {}
