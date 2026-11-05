struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut ret = vec![];
        let s = s.as_bytes();
        let word_count = words.len();
        let chunk_sz = words[0].len();
        let total_len = s.len();

        let mut expected_cnt: HashMap<&[u8], usize> = HashMap::new();
        for w in words.iter() {
            let w = w.as_bytes();
            *expected_cnt.entry(w).or_default() += 1;
        }

        // shift case
        for i in 0..chunk_sz {
            let mut l = i;
            let mut r = i;

            let mut cur_freq: HashMap<&[u8], usize> = HashMap::new();
            let mut valid_word = 0;
            while r + chunk_sz <= total_len {
                let cur_word = &s[r..r + chunk_sz];
                r += chunk_sz;

                if let Some(&expected_freq) = expected_cnt.get(&cur_word) {
                    *cur_freq.entry(cur_word).or_default() += 1;
                    valid_word += 1;
                    while cur_freq[&cur_word] > expected_freq {
                        let delete_word = &s[l..l + chunk_sz];
                        l += chunk_sz;
                        valid_word -= 1;
                        *cur_freq.get_mut(&delete_word).unwrap() -= 1;
                    }
                    if valid_word == word_count {
                        ret.push(l as i32);
                    }
                } else {
                    l = r;
                    cur_freq.clear();
                    valid_word = 0;
                }
            }
        }

        ret
    }
}

fn main() {}
