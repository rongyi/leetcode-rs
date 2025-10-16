struct Solution;

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .map(|mut v| {
                let mut ret = vec![];
                while v != 0 {
                    ret.push(v % 10);
                    v /= 10;
                }
                ret.reverse();
                ret
            })
            .fold(vec![], |mut acc, cur| {
                acc.extend(cur);
                acc
            })
    }
}

fn main() {}
