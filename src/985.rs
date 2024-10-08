struct Solution;

impl Solution {
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sum: i32 = nums.iter().filter(|&&x| x % 2 == 0).sum();
        let mut ret: Vec<i32> = Vec::with_capacity(queries.len());

        for q in queries.into_iter() {
            let index = q[1] as usize;
            let val = q[0];

            if nums[index] % 2 == 0 {
                sum -= nums[index];
            }
            nums[index] += val;
            if nums[index] % 2 == 0 {
                sum += nums[index];
            }
            ret.push(sum);
        }

        ret
    }
}

fn main() {}
