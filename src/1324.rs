#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn print_vertically(s: String) -> Vec<String> {
        let chunks: Vec<Vec<char>> = s.split(' ').map(|s| s.chars().collect()).collect();
        let m = chunks.len();
        let n = chunks.iter().map(|r| r.len()).max().unwrap();
        let mut ret = Vec::new();
        for j in 0..n {
            let mut cur = String::new();
            for i in 0..m {
                if j >= chunks[i].len() {
                    cur.push(' ');
                } else {
                    cur.push(chunks[i][j]);
                }
            }
            ret.push(cur.trim_end().to_owned());
        }
        ret
    }
}

fn main() {}
