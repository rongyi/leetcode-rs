struct Solution;

impl Solution {
    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let sz = queries.len();
        let mut qi: Vec<(usize, i32)> = queries.into_iter().enumerate().collect();
        qi.sort_by_key(|v| v.1);
        nums.sort_unstable();
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        let mut ret = vec![0; sz];
        for (origin_index, query_val) in qi.into_iter() {
            let cur_p = nums.partition_point(|v| *v <= query_val);
            ret[origin_index] = cur_p as i32;
        }

        ret
    }
}

fn main() {}
