struct Solution;

impl Solution {
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        Self::merge_sort(&mut nums, 0, sz - 1)
    }

    fn merge_sort(nums: &mut Vec<i32>, s: usize, e: usize) -> i32 {
        if s >= e {
            return 0;
        }
        let mid = s + (e - s) / 2;

        let mut cnt = Self::merge_sort(nums, s, mid) + Self::merge_sort(nums, mid + 1, e);
        // mid in left
        let mut j = mid + 1;
        for i in s..=mid {
            while j <= e && nums[i] as f64 / 2.0 > nums[j] as f64 {
                j += 1;
            }
            cnt += (j - (mid + 1)) as i32;
        }

        // let mut sorted: Vec<i32> = nums[s..=e].iter().map(|i| i.to_owned()).collect();
        // sorted.sort();
        nums[s..=e].sort();

        cnt
    }
}

fn main() {}
