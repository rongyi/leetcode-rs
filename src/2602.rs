struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
        nums.sort_unstable();
        let sz = nums.len();
        let mut prefix = vec![0i64; sz + 1];
        for (i, &num) in nums.iter().enumerate() {
            prefix[i + 1] = prefix[i] + num as i64;
        }
        let mut ret = vec![];

        for &q in queries.iter() {
            let idx = nums.partition_point(|&x| x < q);
            let val = q as i64 * (2 * idx as i64 - sz as i64) + prefix[sz] - 2 * prefix[idx];
            ret.push(val);
        }

        ret
    }
}

fn main() {}
