#![allow(dead_code)]

struct Solution;

impl Solution {
    // pick small one
    // pick bigger one
    // repeat
    pub fn sort_string(s: String) -> String {
        let mut cnt = vec![0; 26];
        let s: Vec<char> = s.chars().collect();
        let mut sz = s.len() as i32;
        let mut ret = String::new();

        for &c in s.iter() {
            cnt[c as usize - 'a' as usize] += 1;
        }
        while sz > 0 {
            for (idx, cur_cnt) in cnt.iter_mut().enumerate() {
                if *cur_cnt > 0 {
                    ret.push(('a' as u8 + idx as u8) as char);
                    *cur_cnt -= 1;
                    sz -= 1;
                }
            }
            // note the index
            for (idx, cur_cnt) in cnt.iter_mut().enumerate().rev() {
                if *cur_cnt > 0 {
                    ret.push(('a' as u8 + idx as u8) as char);
                    *cur_cnt -= 1;
                    sz -= 1;
                }
            }
        }

        ret
    }
}

fn main() {}
