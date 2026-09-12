struct Solution;

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let sum: i32 = arr.iter().sum();
        if sum % 3 != 0 {
            return false;
        }
        let mut part = 0;
        let mut cur_sum = 0;
        let target = sum / 3;

        for &num in arr.iter() {
            cur_sum += num;
            if cur_sum == target {
                part += 1;
                cur_sum = 0;
            }
        }
        part >= 3
    }
}

fn main() {}
