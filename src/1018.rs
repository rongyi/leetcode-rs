struct Solution;

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut sum = 0;
        let mut ret = Vec::new();
        for &num in &nums {
            sum = sum * 2 + num;
            if sum % 5 == 0 {
                ret.push(true);
            } else {
                ret.push(false);
            }
            // important, incase overflow
            sum %= 5;
        }
        ret
    }
}

fn main() {
    let nums = vec![1, 1, 1, 0];
    let mut sum = 0;
    for &num in &nums {
        sum = sum * 2 + num;
    }
    println!("{}", sum);
}
