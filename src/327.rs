struct Solution;

impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let sz = nums.len();
        let mut prefix = vec![0; sz + 1];
        for i in 0..sz {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }
        // to copy from this sorted tmp to prefix
        let mut tmp = vec![0i64; sz + 1];
        let (lower, upper) = (lower as i64, upper as i64);

        Self::merge_sort(&mut prefix, 0, sz + 1, lower, upper, &mut tmp)
    }

    fn merge_sort(
        nums: &mut Vec<i64>,
        start: usize,
        end: usize,
        lower: i64,
        upper: i64,
        tmp: &mut Vec<i64>,
    ) -> i32 {
        if end - start <= 1 {
            return 0;
        }
        let mid = start + (end - start) / 2;
        let mut ret = Self::merge_sort(nums, start, mid, lower, upper, tmp)
            + Self::merge_sort(nums, mid, end, lower, upper, tmp);
        // from now on, right half mid..end is sorted

        let mut lo = mid;
        let mut hi = mid;
        for i in start..mid {
            // skip small
            while lo < end && nums[i] + lower > nums[lo] {
                lo += 1;
            }
            // sliding window
            while hi < end && nums[i] + upper >= nums[hi] {
                hi += 1;
            }
            ret += (hi - lo) as i32
        }

        // merge action
        let mut i = start;
        let mut j = mid;
        let mut k = start;
        while i < mid && j < end {
            if nums[i] <= nums[j] {
                tmp[k] = nums[i];
                i += 1;
            } else {
                tmp[k] = nums[j];
                j += 1;
            }

            k += 1;
        }
        while i < mid {
            tmp[k] = nums[i];
            i += 1;
            k += 1;
        }
        while j < end {
            tmp[k] = nums[j];
            j += 1;
            k += 1;
        }
        nums[start..end].copy_from_slice(&tmp[start..end]);

        ret
    }
}

fn main() {}
