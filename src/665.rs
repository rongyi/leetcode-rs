struct Solution;

impl Solution {
    // 顺序检查凹变段和逆序检查凸变段。
    // 如果满足，则asc和desc中的较小值必然不大于1。
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut from_left = 0;
        let mut from_right = 0;
        let mut max_sofar = i32::MIN;
        for &num in nums.iter() {
            if num >= max_sofar {
                max_sofar = num;
            } else {
                from_left += 1;
            }
        }
        let mut min_sofar = i32::MAX;
        for &num in nums.iter().rev() {
            if num <= min_sofar {
                min_sofar = num;
            } else {
                from_right += 1;
            }
        }

        !(from_left > 1 && from_right > 1)
    }
}

fn main() {}
