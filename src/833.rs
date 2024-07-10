#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn find_replace_string(
        s: String,
        indices: Vec<i32>,
        sources: Vec<String>,
        targets: Vec<String>,
    ) -> String {
        let s: Vec<char> = s.chars().collect();
        let source: Vec<Vec<char>> = sources.into_iter().map(|s| s.chars().collect()).collect();
        let targets: Vec<Vec<char>> = targets.into_iter().map(|s| s.chars().collect()).collect();
        let mut groups = Vec::new();
        for i in 0..source.len() {
            groups.push((indices[i] as usize, source[i].clone(), targets[i].clone()));
        }

        groups.sort();

        let mut out: Vec<char> = Vec::new();
        let mut i = 0;
        let sz = s.len();

        for (j, cmp, replace) in groups.into_iter() {
            while i < j {
                out.push(s[i]);
                i += 1;
            }
            let cmpsz = cmp.len();
            let mut k = j;
            let mut l = 0;
            while k < sz && l < cmpsz && s[k] == cmp[l] {
                k += 1;
                l += 1;
            }
            // fully matched with cmp
            if l == cmpsz {
                // we use replace
                out.extend(&replace);
                i += cmpsz;
            } else {
                // do nothing let the beginning while to suck from origin string
            }
        }

        while i < sz {
            out.push(s[i]);
            i += 1;
        }

        out.into_iter().collect()
    }
}

fn main() {}
