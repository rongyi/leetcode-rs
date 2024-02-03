struct Solution;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut ret = 0;

        let msks: Vec<u32> = words
            .iter()
            .map(|w| {
                w.chars()
                    .fold(0, |acc, c| acc | (1 << (c as u8 - '0' as u8)))
            })
            .collect();

        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if msks[i] & msks[j] == 0 {
                    let cur = words[i].len() * words[j].len();
                    ret = ret.max(cur);
                }
            }
        }

        ret as i32
    }
}

fn main() {}
