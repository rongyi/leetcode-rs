struct Solution;

use std::collections::HashSet;
use std::collections::VecDeque;

// translated from cpp
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let begin_word: Vec<char> = begin_word.chars().collect();
        let end_word: Vec<char> = end_word.chars().collect();
        let word_list: Vec<Vec<char>> = word_list
            .into_iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        let mut word_set: HashSet<Vec<char>> = word_list.into_iter().collect();
        if !word_set.contains(&end_word) {
            return 0;
        }

        // using bfs to get the shorted path
        let mut q: VecDeque<(Vec<char>, i32)> = VecDeque::new();
        q.push_back((begin_word.clone(), 1));

        while !q.is_empty() {
            let (cur, cur_depth) = q.pop_front().unwrap();
            if cur == end_word {
                return cur_depth;
            }
            let neibs = Self::collect(&cur, &mut word_set);
            for neib in neibs.iter() {
                q.push_back((neib.clone(), cur_depth + 1));
            }
        }

        return 0;
    }

    fn collect(s: &Vec<char>, dict: &mut HashSet<Vec<char>>) -> Vec<Vec<char>> {
        let mut ret = Vec::new();
        for i in 0..s.len() {
            let mut cp = s.clone();
            for j in 0..26 {
                if cp[i] == (('a' as u8) + j) as char {
                    continue;
                }
                cp[i] = ('a' as u8 + j) as char;
                if dict.contains(&cp) {
                    ret.push(cp.clone());
                    dict.remove(&cp);
                }
            }
        }
        ret
    }
}

fn main() {
    let word_list = vec!["hot", "dot", "dog", "lot", "log", "cog"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let s = Solution::find_ladders("hit".to_string(), "cog".to_string(), word_list);
    println!("{:?}", s);
}
