#![allow(dead_code)]
struct Solution;

impl Solution {
    // notice the word rearrange, this means we only need to count char number
    // in this interval
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let s: Vec<char> = s.chars().collect();
        let mut prefix = vec![];
        let mut cur = vec![0; 26];
        prefix.push(cur.clone());

        for &c in s.iter() {
            let idx = (c as u8 - 'a' as u8) as usize;
            cur[idx] += 1;
            prefix.push(cur.clone());
        }
        let mut ret = Vec::new();
        for q in queries.into_iter() {
            let (lo, hi, k) = (q[0], q[1], q[2]);
            let mut cur_sum = prefix[hi as usize + 1].clone();

            let mut diff = 0;
            let len = hi - lo + 1;
            for i in 0..26 {
                cur_sum[i] -= prefix[lo as usize][i];
                diff += if cur_sum[i] % 2 == 1 { 1 } else { 0 };
            }
            diff -= if len % 2 == 1 { 1 } else { 0 };
            ret.push(if diff <= k * 2 { true } else { false });
        }

        ret
    }
}

fn main() {}
