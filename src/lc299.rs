struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut bull = 0;
        let mut cow = 0;
        let s: Vec<char> = secret.chars().collect();
        let g: Vec<char> = guess.chars().collect();
        let mut cnt: HashMap<char, i32> = HashMap::new();

        for (i, &sc) in s.iter().enumerate() {
            let gc = g[i];
            if sc == gc {
                bull += 1;
            } else {
                *cnt.entry(sc).or_insert(0) += 1;
            }
        }
        for (i, &gc) in g.iter().enumerate() {
            let sc = s[i];
            if sc != gc && cnt.contains_key(&gc) {
                cow += 1;
                let e = cnt.get_mut(&gc).unwrap();
                *e -= 1;
                if *e == 0 {
                    cnt.remove(&gc);
                }
            }
        }

        format!("{}A{}B", bull, cow)
    }
}

fn main() {}
