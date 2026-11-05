struct Solution;

impl Solution {
    pub fn two_city_sched_cost(mut costs: Vec<Vec<i32>>) -> i32 {
        costs.sort_by(|l, r| (l[0] - l[1]).cmp(&(r[0] - r[1])));
        let sz = costs.len();
        let mut ret = 0;

        for i in 0..sz / 2 {
            ret += costs[i][0] + costs[i + sz / 2][1];
        }

        ret
    }
}

fn main() {}
