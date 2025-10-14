struct Solution;

impl Solution {
    pub fn maximum_tastiness(mut price: Vec<i32>, k: i32) -> i32 {
        price.sort_unstable();
        let mut l = 0;
        let mut r = 1e9 as i32;

        while l < r {
            let mid = l + (r - l) / 2;
            if Self::check(mid, &price, k) {
                l = mid + 1;
            } else {
                r = mid;
            }
        }

        l - 1
    }

    fn check(cur: i32, price: &Vec<i32>, k: i32) -> bool {
        let mut cnt = 1;
        let mut last_idx = 0;
        let mut cur_idx = 1;
        while cur_idx < price.len() && cnt < k {
            if price[cur_idx] - price[last_idx] >= cur {
                last_idx = cur_idx;
                cnt += 1;
            }

            cur_idx += 1;
        }

        cnt == k
    }
}

fn main() {}
