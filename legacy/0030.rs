struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let wlen = words[0].len();
        let total = s.len();
        let word_count = words.len();
        let mut ret = Vec::new();
        if total == 0 || word_count == 0 || total < word_count * wlen {
            return ret;
        }
        let mut word_freq = HashMap::new();
        // word frequency count
        for w in &words {
            *word_freq.entry(w.to_owned()).or_insert(0) += 1;
        }

        for i in 0..wlen {
            let mut word_seen = HashMap::new();
            let mut j = i;
            let mut count = 0;
            while j <= total - wlen {
                let cur = &s[j..(j + wlen)].to_owned();

                if word_freq.contains_key(cur) {
                    *word_seen.entry(cur.to_owned()).or_insert(0) += 1;
                    count += 1;

                    while *word_seen.get(cur).unwrap() > *word_freq.get(cur).unwrap() {
                        let start = j - (count - 1) * wlen;
                        let left_most = &s[start..start + wlen].to_owned();
                        *word_seen.get_mut(left_most).unwrap() -= 1;
                        count -= 1;
                    }
                    if count == word_count {
                        ret.push((j - (count - 1) * wlen) as i32);
                    }
                } else {
                    word_seen.clear();
                    count = 0;
                }

                j += wlen;
            }
        }

        ret
    }
}
