struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let mut words = words;
        words.sort_by(|l, r| l.len().cmp(&r.len()));
        let mut pre_words: HashSet<String> = HashSet::new();

        let mut ret: Vec<String> = Vec::new();
        for word in words.iter() {
            if Self::can_form(word, &pre_words) {
                ret.push(word.clone());
            }
            pre_words.insert(word.clone());
        }

        ret
    }
    fn can_form(s: &str, dict: &HashSet<String>) -> bool {
        let sz = s.len();
        if s.is_empty() {
            return false;
        }
        // "abcd" 4 个字符 有5 个gap
        // |a|b|c|d|  竖线地方就是下刀的地方
        let mut dp = vec![false; sz + 1];
        // 任何字符都可以由 "" + 自己构成，所以切最左边第一刀，总是可以。
        dp[0] = true;
        // 因为必须是非空字符构成，从1开始
        for i in 1..sz + 1 {
            for j in 0..i {
                if !dp[j] {
                    continue;
                }
                if dict.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[sz]
    }
}

fn main() {}
