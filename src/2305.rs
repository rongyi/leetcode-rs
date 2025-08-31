struct Solution;

use std::i32;
impl Solution {
    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        let mut kids = vec![0; k as usize];
        let mut ret = i32::MAX;
        Self::dfs(&cookies, &mut kids, 0, &mut ret);

        ret
    }
    fn dfs(cookies: &Vec<i32>, kids: &mut Vec<i32>, cur_idx: usize, ret: &mut i32) {
        if cur_idx == cookies.len() {
            let val = *kids.iter().max().unwrap();
            *ret = (*ret).min(val);
            return;
        }
        for i in 0..kids.len() {
            kids[i] += cookies[cur_idx];
            Self::dfs(cookies, kids, cur_idx + 1, ret);

            kids[i] -= cookies[cur_idx];
            if kids[i] == 0 {
                break;
            }
        }
    }
}
fn main() {}
