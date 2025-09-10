struct Solution;


impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let sz = nums.len();
        let mut acc = 0;
        for i in 0..sz {
            for j in i + 1..sz {
                for k in j + 1..sz {
                    if nums[j] - nums[i] == diff && nums[k] - nums[j] == diff {
                        acc += 1;
                    }
                }
            }
        }
        acc
    }
}

fn main() {}
