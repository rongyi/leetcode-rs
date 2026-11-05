struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

// translated from cpp
impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let begin_word: Vec<char> = begin_word.chars().collect();
        let end_word: Vec<char> = end_word.chars().collect();
        let word_list: Vec<Vec<char>> = word_list
            .into_iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        let mut word_set: HashSet<Vec<char>> = word_list.into_iter().collect();

        let mut q: VecDeque<Vec<char>> = VecDeque::new();
        q.push_back(begin_word.clone());

        let mut depth_cache: HashMap<Vec<char>, i32> = HashMap::new();
        depth_cache.insert(begin_word.clone(), 1);

        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            let neibs = Self::collect(&cur, &mut word_set);
            for neib in neibs.iter() {
                q.push_back(neib.clone());
                if !depth_cache.contains_key(neib) {
                    depth_cache.insert(neib.clone(), depth_cache.get(&cur).unwrap() + 1);
                }
            }
        }

        let mut ret = Vec::new();

        if depth_cache.contains_key(&end_word) {
            Self::dfs(&begin_word, end_word, &depth_cache, &mut ret, Vec::new());
        }

        ret.into_iter()
            .map(|vv| vv.into_iter().map(|v| v.into_iter().collect()).collect())
            .collect()
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

    fn dfs(
        w1: &Vec<char>,
        cur_end: Vec<char>,
        dc: &HashMap<Vec<char>, i32>,
        ret: &mut Vec<Vec<Vec<char>>>,
        curpath: Vec<Vec<char>>,
    ) {
        let mut curpath = curpath;
        curpath.push(cur_end.clone());
        if cur_end == *w1 {
            curpath.reverse();
            ret.push(curpath.clone());
            return;
        }

        let d = dc.get(&cur_end).unwrap();

        for i in 0..w1.len() {
            let mut cp = cur_end.clone();
            for j in 0..26 {
                cp[i] = ('a' as u8 + j) as char;
                if dc.contains_key(&cp) && *dc.get(&cp).unwrap() == d - 1 {
                    Self::dfs(w1, cp.clone(), dc, ret, curpath.clone());
                }
            }
        }
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
