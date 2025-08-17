struct Solution;

impl Solution {
    pub fn count_hill_valley(origin: Vec<i32>) -> i32 {
        // dedup first
        let mut nums = vec![];
        for num in origin {
            if nums.is_empty() || *nums.last().unwrap() != num {
                nums.push(num);
            }
        }
        let sz = nums.len();
        if sz < 3 {
            return 0;
        }
        let mut ret = 0;
        for i in 1..sz - 1 {
            if nums[i] > nums[i - 1] && nums[i] > nums[i + 1] {
                ret += 1;
            }
            if nums[i] < nums[i - 1] && nums[i] < nums[i + 1] {
                ret += 1;
            }
        }
        ret
    }
}

fn main() {}
