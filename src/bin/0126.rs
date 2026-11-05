struct Solution;

use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut dict: HashSet<String> = word_list.into_iter().collect();

        let mut q = VecDeque::new();
        q.push_back(begin_word.clone());

        let mut depth_cache: HashMap<String, i32> = HashMap::new();
        depth_cache.insert(begin_word.clone(), 1);

        let mut found = false;
        while !q.is_empty() && !found {
            let sz = q.len();
            for _ in 0..sz {
                let cur = q.pop_front().unwrap();
                let mut cur_word: Vec<char> = cur.chars().collect();
                let cur_depth = *depth_cache.get(&cur).unwrap();
                for i in 0..cur_word.len() {
                    let origin = cur_word[i];
                    // change to find path
                    for c in 'a'..='z' {
                        if c == origin {
                            continue;
                        }
                        cur_word[i] = c;
                        let next: String = cur_word.iter().collect();
                        if next == end_word {
                            found = true;
                        }

                        if dict.contains(&next) {
                            // to make sure only the first shorted path in
                            if !depth_cache.contains_key(&next) {
                                depth_cache.insert(next.clone(), cur_depth + 1);
                            }
                            // this is bfs, we dont revisit it again, so delete this in table
                            dict.remove(&next);
                            q.push_back(next);
                        }
                    }

                    cur_word[i] = origin;
                }
            }
        }

        // println!("{:?}", depth_cache);

        //  now we have depth table we using bfs to search from end to start
        let mut out = vec![];
        if depth_cache.contains_key(&end_word) {
            let mut cur_path = vec![end_word.clone()];
            Self::backtrack(
                &begin_word,
                &end_word,
                &mut cur_path,
                &mut out,
                &depth_cache,
                *depth_cache.get(&end_word).unwrap(),
            );
        }

        out
    }

    fn backtrack(
        begin_world: &str,
        end_word: &str,
        cur_path: &mut Vec<String>,
        out: &mut Vec<Vec<String>>,
        depth_cache: &HashMap<String, i32>,
        cur_depth: i32,
    ) {
        if end_word == begin_world {
            let mut ordered_path = cur_path.clone();
            ordered_path.reverse();
            out.push(ordered_path);
            return;
        }
        let mut end_vec: Vec<char> = end_word.chars().collect();

        for i in 0..end_vec.len() {
            let origin = end_vec[i];
            for c in 'a'..='z' {
                if c == origin {
                    continue;
                }
                end_vec[i] = c;

                let upper_next: String = end_vec.iter().collect();
                if depth_cache.contains_key(&upper_next)
                    && *depth_cache.get(&upper_next).unwrap() == cur_depth - 1
                {
                    cur_path.push(upper_next.clone());

                    Self::backtrack(
                        begin_world,
                        &upper_next,
                        cur_path,
                        out,
                        depth_cache,
                        cur_depth - 1,
                    );

                    cur_path.pop();
                }
            }
            end_vec[i] = origin;
        }
    }
}

fn main() {
    let start = "hit".to_string();
    let end = "cog".to_string();
    let word_list: Vec<String> = ["hot", "dot", "dog", "lot", "log", "cog"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    Solution::find_ladders(start, end, word_list);
}
