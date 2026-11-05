struct Solution;
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let mut dict: HashSet<String> = bank.into_iter().collect();
        if !dict.contains(&end) {
            return -1;
        }
        dict.insert(start.clone());
        let mut q: VecDeque<String> = VecDeque::new();
        q.push_back(start);

        let mut ret = 0;
        while !q.is_empty() {
            let sz = q.len();
            for i in 0..sz {
                let cur = q.pop_front().unwrap();
                if cur == end {
                    return ret;
                }
                Self::edit(&cur, &mut dict, &mut q);
            }

            ret += 1;
        }

        -1
    }

    fn edit(cur: &str, dict: &mut HashSet<String>, q: &mut VecDeque<String>) {
        // clear cur in dict to make it not go this way
        dict.remove(cur);

        let cur: Vec<char> = cur.chars().collect();
        for i in 0..cur.len() {
            let mut ch = cur.clone();
            for c in ['A', 'C', 'G', 'T'] {
                ch[i] = c;
                let edit_gen: String = ch.iter().collect();
                if dict.contains(&edit_gen) {
                    q.push_back(edit_gen.clone());
                    dict.remove(&edit_gen);
                }
            }
        }
    }
}

fn main() {}
