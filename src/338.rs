struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ret = vec![0; n as usize + 1];

        for i in 0..=n {
            let ones = i.count_ones();
            ret[i as usize] = ones as i32;
        }

        ret
    }
}
fn main() {}
