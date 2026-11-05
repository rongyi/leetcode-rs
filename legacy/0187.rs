struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut cnt: HashMap<&str, usize> = HashMap::new();
        let sz = s.len();
        let mut ret: Vec<String> = Vec::new();
        if sz < 10 {
            return ret;
        }

        for i in 0..=sz - 10 {
            let chunk = &s[i..i + 10];
            if let Some(e) = cnt.get_mut(&chunk) {
                if *e == 1 {
                    ret.push(chunk.to_owned());
                }
                *e += 1;
            } else {
                cnt.insert(chunk, 1);
            }
        }

        ret
    }
}

fn main() {}
