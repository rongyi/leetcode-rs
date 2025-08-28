struct Solution;

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut ret = 0;
        for i in 0..31 {
            let mut cur_acc = 0;
            for &v in candidates.iter() {
                // how many 1 in this ith position?
                if ((1 << i) & v) != 0 {
                    cur_acc += 1;
                }
            }
            ret = ret.max(cur_acc)
        }

        ret
    }
}

fn main() {}
