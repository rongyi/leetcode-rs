struct Solution;

impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let odd_nums: Vec<i32> = nums.iter().filter(|&&x| x % 2 == 1).copied().collect();
        let even_nums: Vec<i32> = nums.iter().filter(|&&x| x % 2 == 0).copied().collect();
        let sz = nums.len();
        let mut ret: Vec<i32> = vec![0; sz];
        for (i, j) in (0..sz).step_by(2).zip(0..) {
            ret[i] = even_nums[j];
        }
        for (i, j) in (1..sz).step_by(2).zip(0..) {
            ret[i] = odd_nums[j];
        }

        ret
    }
}

fn main() {}
