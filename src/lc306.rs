struct Solution;

impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        let n = num.len();
        if n <= 2 {
            return false;
        }
        let nums: Vec<u64> = num
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();

        // size of the first chunk
        for i in 1..=(n / 2) {
            // println!("outer: {}", nums[i]);
            if nums[0] == 0 && i > 1 {
                break;
            }
            let num1 = nums[..i].iter().fold(0, |acc, &cur| acc * 10 + cur);

            // size of the second chunk
            for j in 1..=(n - i) {
                // nums[i] is the start of the second chunk
                if nums[i] == 0 && j > 1 {
                    break;
                }
                // println!("inner: {}", nums[j]);
                let num2 = nums[i..i + j].iter().fold(0, |acc, &cur| acc * 10 + cur);

                // the false to distill the initial two value without third case
                // e.g. 11 split to 1 + 1, we can not let this case pass
                if Self::is_valid_sequence(&nums[i + j..], num1, num2, false) {
                    return true;
                }
            }
        }

        false
    }

    fn is_valid_sequence(slice: &[u64], num1: u64, num2: u64, summed: bool) -> bool {
        if slice.is_empty() {
            return summed;
        }
        let sum = num1 + num2;
        let sum_str = sum.to_string();
        let sum_len = sum_str.len();
        if slice.len() < sum_len {
            return false;
        }
        if slice[..sum_len]
            .iter()
            .map(|&d| d.to_string())
            .collect::<String>()
            != sum_str
        {
            return false;
        }
        Self::is_valid_sequence(&slice[sum_len..], num2, sum, true)
    }
}

fn main() {
    Solution::is_additive_number("10".to_string());
}
