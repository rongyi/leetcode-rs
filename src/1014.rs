struct Solution;

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        // minus 1 to indicate the initial gap
        let mut maxi = values[0] - 1;
        let mut ret = 0;
        for j in 1..values.len() {
            ret = ret.max(values[j] + maxi);
            maxi = maxi.max(values[j]);

            // their gap is always increase
            // two case:
            // 1. current j is the new max num
            //    minus 1 to indicate the intial gap: 1
            // 2. some prev num is still the max start
            //    minux 1 to increase the gap
            maxi -= 1;
        }
        ret
    }
}

fn main() {}
