struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        // index:  0, 1, 2, 3, 4
        // expect: 1, 2, 3, 4, 5
        // bucket sort, like pegeon hole
        let sz = nums.len();
        for i in 0..sz {
            while nums[i] > 0 && nums[i] <= sz as i32 && nums[nums[i] as usize - 1] != nums[i] {
                let target_idx = nums[i] as usize - 1;
                nums.swap(target_idx, i);
            }
        }

        for i in 0..sz {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as _;
            }
        }

        (sz + 1) as _
    }
}

fn main() {}
