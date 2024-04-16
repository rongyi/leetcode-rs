#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let mut char_cnt = vec![vec![0; 26]; stickers.len()];

        for (i, cur_sticker) in stickers.iter().enumerate() {
            for c in cur_sticker.chars() {
                char_cnt[i][(c as u8 - 'a' as u8) as usize] += 1;
            }
        }

        let mut dp: HashMap<String, i32> = HashMap::new();
        dp.insert("".to_string(), 0);

        Self::recur(&mut dp, &mut char_cnt, &target)
    }
    fn recur(dp: &mut HashMap<String, i32>, char_cnt: &Vec<Vec<i32>>, target: &str) -> i32 {
        if dp.contains_key(target) {
            return dp[target];
        }

        let mut ret = i32::MAX;

        let mut cur_cnt = vec![0; 26];
        for c in target.chars() {
            cur_cnt[(c as u8 - 'a' as u8) as usize] += 1;
        }
        let indexed_target: Vec<char> = target.chars().collect();
        // try to use stickers of index i
        for i in 0..char_cnt.len() {
            if char_cnt[i][(indexed_target[0] as u8 - 'a' as u8) as usize] == 0 {
                continue;
            }
            let mut s: String = String::new();
            for j in 0..26 {
                if cur_cnt[j] - char_cnt[i][j] > 0 {
                    let c = ('a' as u8 + j as u8) as char;
                    let left_part = c.to_string().repeat((cur_cnt[j] - char_cnt[i][j]) as usize);
                    s.push_str(&left_part);
                }
            }
            println!("{:?}", s);
            let tmp = Self::recur(dp, char_cnt, &s);
            if tmp != -1 {
                ret = ret.min(1 + tmp);
            }
        }
        dp.insert(target.to_string(), if ret == i32::MAX { -1 } else { ret });

        *dp.get(&target.to_string()).unwrap()
    }
}

fn main() {
    let input = ["with", "example", "science"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let target = "thehat".to_string();
    let val = Solution::min_stickers(input, target);
    println!("{}", val);
}
