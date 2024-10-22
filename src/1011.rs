struct Solution;

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut r: i32 = weights.iter().sum();
        let mut l = (*weights.iter().max().unwrap()).max(r / days);
        while l < r {
            let m = l + (r - l) / 2;
            // yes, we can ship within days d, so we want to try smaller ship case
            if Self::count_days(&weights, m) <= days {
                r = m;
            } else {
                // else can not deliver within days, make ship larger
                l = m + 1;
            }
        }
        l
    }
    fn count_days(ws: &Vec<i32>, total_cap: i32) -> i32 {
        let mut ret = 1;
        let mut cur_cap = 0;
        for &w in ws.iter() {
            cur_cap += w;
            if cur_cap > total_cap {
                ret += 1;
                cur_cap = w;
            }
        }
        ret
    }
}

fn main() {}
