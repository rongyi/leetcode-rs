struct Solution;

impl Solution {
    pub fn sum_subarray_mins(nums: Vec<i32>) -> i32 {
        let nums: Vec<i64> = nums.into_iter().map(|i| i as i64).collect();

        let sz = nums.len();
        let mut prev_less = Vec::new();
        let mut next_less = Vec::new();
        let mut left = vec![0i64; sz];
        let mut right = vec![0i64; sz];

        for i in 0..sz {
            left[i] = i as i64 + 1;
        }
        for i in 0..sz {
            right[i] = (sz - i) as i64;
        }

        for i in 0..sz {
            while !prev_less.is_empty() && nums[*prev_less.last().unwrap()] > nums[i] {
                prev_less.pop();
            }
            left[i] = if prev_less.is_empty() {
                i as i64 + 1
            } else {
                (i - *prev_less.last().unwrap()) as i64
            };
            prev_less.push(i);

            while !next_less.is_empty() && nums[*next_less.last().unwrap()] > nums[i] {
                let x = next_less.pop().unwrap();
                right[x] = (i - x) as i64;
            }
            next_less.push(i);
        }
        let mut ret: i64 = 0;
        let m = 1e9 as i64 + 7;
        for i in 0..sz {
            ret = (ret + nums[i] * left[i] * right[i]) % m;
        }

        ret as i32
    }
}

fn main() {}
