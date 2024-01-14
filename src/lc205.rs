
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let mut smap: HashMap<char, char> = HashMap::new();
        let mut tmap: HashMap<char, char> = HashMap::new();

        if s.len() != t.len() {
            return false;
        }

        for i in 0..s.len() {
            match (smap.get(&s[i]), tmap.get(&t[i])) {
                (Some(&tot), _) if tot != t[i] => {
                    return false;
                }
                (_, Some(&tos)) if tos != s[i] => {
                    return false;
                }
                _ => {
                    // bind the relationship
                    smap.insert(s[i], t[i]);
                    tmap.insert(t[i], s[i]);
                }
            }
        }

        true
    }
}

fn main() {}
