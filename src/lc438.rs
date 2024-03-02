struct Solution;
use std::collections::HashMap;


impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        let mut ret = Vec::new();
        let mut pfreq: HashMap<char, i32> = HashMap::new();
        let mut sfreq: HashMap<char, i32> = HashMap::new();

        for &c in p.iter() {
            *pfreq.entry(c).or_insert(0) += 1;
        }

        let psz = p.len();
        let ssz = s.len();
        let mut left = 0;

        // sliding window
        for right in 0..ssz {
            let ch = s[right];
            *sfreq.entry(ch).or_insert(0) += 1;

            if right >= psz - 1 {
                if sfreq == pfreq {
                    ret.push(left);
                }

                let left_ch = s[left as usize];
                if let Some(cnt) = sfreq.get_mut(&left_ch) {
                    *cnt -= 1;
                    if *cnt == 0 {
                        sfreq.remove(&left_ch);
                    }
                }

                left += 1;
            }
        }

        ret
    }
}

fn main() {}
