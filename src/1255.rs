#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let words: Vec<Vec<char>> = words.into_iter().map(|s| s.chars().collect()).collect();
        let mut cnt: Vec<i32> = vec![0; 26];
        for &c in letters.iter() {
            cnt[(c as u8 - 'a' as u8) as usize] += 1;
        }
        Self::dfs(&words, 0, &mut cnt, &score)
    }
    fn dfs(words: &Vec<Vec<char>>, idx: usize, cnt: &mut Vec<i32>, score: &Vec<i32>) -> i32 {
        if idx >= words.len() {
            return 0;
        }
        let skip_gain = Self::dfs(words, idx + 1, cnt, score);
        let mut valid = true;
        let mut with_gain = 0;

        let mut cur_cnt = cnt.clone();
        for &c in words[idx].iter() {
            cur_cnt[(c as u8 - 'a' as u8) as usize] -= 1;
            if cur_cnt[(c as u8 - 'a' as u8) as usize] < 0 {
                valid = false;
            } else {
                with_gain += score[(c as u8 - 'a' as u8) as usize];
            }
        }
        if valid {
            let next = Self::dfs(words, idx + 1, &mut cur_cnt, score);
            skip_gain.max(with_gain + next)
        } else {
            skip_gain
        }
    }
}

fn main() {}
