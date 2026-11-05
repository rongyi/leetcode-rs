struct Solution;

impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let sz = nums.len();
        let mut num_cost: Vec<(i64, i64)> = nums
            .into_iter()
            .map(|v| v as i64)
            .zip(cost.into_iter().map(|v| v as i64))
            .collect();
        // sort by val
        num_cost.sort_by_key(|a| a.0);
        let mut costl = vec![0i64; sz];
        let mut acc_cost = 0;
        for i in 0..sz - 1 {
            acc_cost += num_cost[i].1;
            costl[i + 1] = costl[i] + acc_cost * (num_cost[i + 1].0 - num_cost[i].0);
        }

        acc_cost = 0;
        let mut ret = *costl.last().unwrap();
        let mut cur_cost = 0;
        for i in (1..sz).rev() {
            acc_cost += num_cost[i].1;
            cur_cost += acc_cost * (num_cost[i].0 - num_cost[i - 1].0);
            ret = ret.min(cur_cost + costl[i - 1]);
        }

        ret
    }
}

fn main() {}
