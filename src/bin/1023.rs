struct Solution;

impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let pattern: Vec<char> = pattern.chars().collect();
        let mut ret = Vec::new();

        for q in queries.into_iter() {
            let q = q.chars().collect::<Vec<char>>();
            let mut i = 0;
            let mut j = 0;
            while i < q.len() {
                if j < pattern.len() && q[i] == pattern[j] {
                    j += 1;
                } else if q[i].is_ascii_uppercase() {
                    break;
                }
                i += 1;
            }
            ret.push(i == q.len() && j == pattern.len());
        }

        ret
    }
}

fn main() {}
