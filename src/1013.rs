struct Solution;

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let sum: i32 = arr.iter().sum();
        if sum % 3 != 0 {
            return false;
        }
        let mut part = 0;
        let mut cur_sum = 0;
        for (i, &num) in arr.iter().enumerate() {
            cur_sum += num;
            if cur_sum == sum / 3 {
                part += 1;
                cur_sum = 0;
            }
            if part == 2 && i != arr.len() - 1 {
                return true;
            }
        }
        false
    }
}

fn main() {}
