struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut index: HashMap<String, usize> = HashMap::new();
        for (i, w) in words.iter().enumerate() {
            let mut w: String = w.chars().rev().collect();
            index.insert(w, i);
        }
        let mut ret = Vec::new();
        if let Some(&empty_index) = index.get("") {
            for (i, w) in words.iter().enumerate() {
                if i == empty_index {
                    continue;
                }
                if Self::is_parlindome(w) {
                    ret.push(vec![i as i32, empty_index as i32]);
                    ret.push(vec![empty_index as i32, i as i32]);
                }
            }
        }
        index.remove("");

        for (i, w) in words.iter().enumerate() {
            let sz = w.len();
            for j in 0..sz {
                let (left, right) = w.split_at(j);
                // xxx left | right
                if let Some(&right_index) = index.get(right) {
                    if Self::is_parlindome(left) && right_index != i {
                        ret.push(vec![right_index as i32, i as i32]);
                    }
                }
                // left | right xxxx
                if let Some(&left_index) = index.get(left) {
                    if Self::is_parlindome(right) && left_index != i {
                        ret.push(vec![i as i32, left_index as i32]);
                    }
                }
            }
        }

        ret
    }

    fn is_parlindome(s: &str) -> bool {
        if s.is_empty() {
            return true;
        }
        let s: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if s[i] != s[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

fn main() {
    let c =  ["abcd","dcba","lls","s","sssll"];
    let a : Vec<String> = c.into_iter().map(String::from).collect();
    let ret = Solution::palindrome_pairs(a);
    println!("{:?}", ret);
}
