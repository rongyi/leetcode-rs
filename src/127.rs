struct Solution;

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut dict: HashSet<String> = word_list.into_iter().collect();
        let mut q = VecDeque::new();
        q.push_back((begin_word, 1));

        while !q.is_empty() {
            let sz = q.len();
            for _ in 0..sz {
                let (cur_s, cur_depth) = q.pop_front().unwrap();
                if cur_s == end_word {
                    return cur_depth;
                }
                let mut cur_vec: Vec<char> = cur_s.chars().collect();
                for i in 0..cur_vec.len() {
                    let origin = cur_vec[i];

                    for c in 'a'..='z' {
                        if c == origin {
                            continue;
                        }
                        cur_vec[i] = c;
                        let next: String = cur_vec.iter().collect();
                        if dict.contains(&next) {
                            dict.remove(&next);
                            q.push_back((next, cur_depth + 1));
                        }
                    }

                    cur_vec[i] = origin;
                }
            }
        }

        0
    }
}

fn main() {}
