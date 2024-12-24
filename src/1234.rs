#![allow(dead_code)]


struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut count: HashMap<char, usize> = HashMap::new();
        let sz = s.len();
        let k = sz / 4;

        let mut ret = sz;
        for &c in s.iter() {
            *count.entry(c).or_insert(0) += 1;
        }
        let mut i = 0;
        for j in 0..sz {
            count.entry(s[j]).and_modify(|v| *v -= 1);
            // 只要保证window外面的统计个数都没有出格的，即 > k，那么window里面调整调整总归是可以配平(to k)的
            while i < sz
                && ['Q', 'W', 'E', 'R']
                    .iter()
                    .all(|c| *count.get(c).unwrap_or(&0) <= k)
            {
                ret = ret.min(j - i + 1);
                count.entry(s[i]).and_modify(|v| *v += 1);
                i += 1;
            }
        }

        ret as i32
    }
}

fn main() {}
