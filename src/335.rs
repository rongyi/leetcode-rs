struct Solution;

impl Solution {
    pub fn is_self_crossing(x: Vec<i32>) -> bool {
        let sz = x.len();
        for i in 3..sz {
            if i >= 3 && x[i] >= x[i - 2] && x[i - 1] <= x[i - 3] {
                return true;
            }
            if i >= 4 && x[i - 1] == x[i - 3] && x[i] + x[i - 4] >= x[i - 2] {
                return true;
            }
            if i >= 5
                && x[i - 2] >= x[i - 4]
                && x[i - 5] + x[i - 1] >= x[i - 3]
                && x[i - 1] <= x[i - 3]
                && x[i - 4] + x[i] >= x[i - 2]
            {
                return true;
            }
        }
        false
    }
}

fn main() {}
