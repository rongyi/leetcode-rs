struct Solution;

// yes, it pass the all the testcase, this is so cool
impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let sz = nums.len();
        let mut sum = vec![0; sz + 1];
        for i in 0..sz {
            sum[i + 1] += sum[i] + nums[i] as i64;
        }

        Self::mergesort(&mut sum, lower, upper, 0, (sz + 1) as i32)
    }
    fn mergesort(sum: &mut Vec<i64>, lower: i32, upper: i32, l: i32, r: i32) -> i32 {
        if r - l <= 1 {
            return 0;
        }
        let mid = (l + r) / 2;
        let mut m = mid;
        let mut n = mid;
        let mut cnt: i32 =
            Self::mergesort(sum, lower, upper, l, mid) + Self::mergesort(sum, lower, upper, mid, r);

        for i in l..mid {
            while m < r && sum[m as usize] - sum[i as usize] < lower as i64 {
                m += 1;
            }

            while n < r && sum[n as usize] - sum[i as usize] <= upper as i64 {
                n += 1;
            }
            cnt += n - m;
        }
        let mut lsta: Vec<i64> = vec![0; (mid - l) as usize];
        let mut lstb: Vec<i64> = vec![0; (r - mid) as usize];
        lsta.copy_from_slice(&sum[l as usize..mid as usize]);
        lstb.copy_from_slice(&sum[mid as usize..r as usize]);
        let mged = Self::merge(lsta, lstb);

        sum[l as usize..r as usize].copy_from_slice(&mged[..]);

        cnt
    }
    fn merge(mut left: Vec<i64>, mut right: Vec<i64>) -> Vec<i64> {
        let mut result = Vec::with_capacity(left.len() + right.len());

        while !left.is_empty() && !right.is_empty() {
            if left[0] <= right[0] {
                result.push(left.remove(0));
            } else {
                result.push(right.remove(0));
            }
        }

        result.extend(left);
        result.extend(right);

        result
    }
}

fn main() {
    let input = vec![-2, 5, -1];
    Solution::count_range_sum(input, -2, 2);
}
