
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_min_step(board: String, hand: String) -> i32 {
        let mut board: Vec<char> = board.chars().collect();
        let hand: Vec<char> = hand.chars().collect();
        let mut freq = vec![0; 26];
        for &c in &hand {
            let idx: usize = (c as u8 - 'A' as u8) as usize;
            freq[idx] += 1;
        }
        let mut cache: HashMap<String, i32> = HashMap::new();

        Self::dfs(board, &mut freq, &mut cache)
    }
    fn dfs(mut board: Vec<char>, freq: &mut Vec<i32>, cache: &mut HashMap<String, i32>) -> i32 {
        let key: String = board.iter().collect::<String>() + "#" + &Self::freq_to_string(freq);
        if cache.contains_key(&key) {
            return cache[&key];
        }
        let mut ret = i32::MAX;
        if board.is_empty() {
            ret = 0;
        } else {
            for i in 0..board.len() {
                for j in 0..freq.len() {
                    let mut worth_try = false;
                    // same color? yes
                    if (board[i] as u8 - 'A' as u8) as usize == j {
                        worth_try = true;
                    } else if i > 0
                        && board[i] == board[i - 1]
                        && ((board[i] as u8 - 'A' as u8) as usize) != j
                    {
                        worth_try = true;
                    }
                    if freq[j] > 0 && worth_try {
                        board.insert(i, ('A' as u8 + j as u8) as char);
                        freq[j] -= 1;

                        let new_board = Self::eliminate_action(&board);
                        let after_steps = Self::dfs(new_board, freq, cache);
                        if after_steps != -1 {
                            ret = ret.min(1 + after_steps);
                        }

                        freq[j] += 1;
                        board.remove(i);
                    }
                }
            }
        }

        if ret == i32::MAX {
            ret = -1;
        }
        cache.insert(key, ret);
        ret
    }

    fn eliminate_action(board: &Vec<char>) -> Vec<char> {
        let mut start = 0;
        let mut end = 0;
        let sz = board.len();
        while start < sz {
            while end < sz && board[start] == board[end] {
                end += 1;
            }
            if end - start >= 3 {
                let new_board: Vec<char> = board[0..start]
                    .iter()
                    .chain(board[end..].iter())
                    .map(|c| c.to_owned())
                    .collect();
                return Self::eliminate_action(&new_board);
            } else {
                // can not elinmate the current word
                start = end;
            }
        }

        board.to_owned()
    }

    fn freq_to_string(freq: &Vec<i32>) -> String {
        let mut ret = String::new();
        for i in 0..freq.len() {
            if freq[i] > 0 {
                ret.push(('A' as u8 + i as u8) as char);
                ret.push_str(&freq[i].to_string());
            }
        }

        ret
    }
}

fn main() {
    // let board = "WRRBBW".to_string();
    // let hand = "RB".to_string();
    // println!("Minimum steps: {}", Solution::find_min_step(board, hand)); // Output: 2
    let a = vec!['a', 'b', 'c'];
    let b: String = a.iter().collect::<String>() + "#$" + "hello";
    println!("{}", b);
}
