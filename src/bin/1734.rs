#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let n = encoded.len() as i32 + 1;
        let all = (1..=n).fold(0, |acc, cur| acc ^ cur);

        let xall = encoded.iter().step_by(2).fold(0, |acc, cur| acc ^ cur);
        let mut ret = Vec::with_capacity(n as usize);
        ret.push(all ^ xall);

        for &x in encoded.iter().rev() {
            ret.push(*ret.last().unwrap() ^ x);
        }
        ret.reverse();

        ret
    }
}
fn main() {}
