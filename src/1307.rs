#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        let words: Vec<Vec<char>> = words
            .into_iter()
            .map(|s| s.chars().rev().collect())
            .collect();
        let result: Vec<char> = result.chars().rev().collect();

        if words.iter().any(|s| s.len() > result.len()) {
            return false;
        }

        let mut char_to_d = vec![-1; 256];
        let mut visited = vec![false; 10];

        Self::dfs(&words, &result, &mut char_to_d, &mut visited, 0, 0, 0)
    }

    // result -> right
    // words -> left
    // sum -> words sum
    // i means row or just the index in words
    // j -> result process index
    fn dfs(
        words: &Vec<Vec<char>>,
        result: &Vec<char>,
        ctod: &mut Vec<i32>,
        dtoc: &mut Vec<bool>,
        j: usize,
        i: usize,
        sum: i32,
    ) -> bool {
        // the final right case
        if j == result.len() {
            if sum != 0 {
                return false;
            }
            // now sum is 0, but we need to process the leading zero invalid case
            if result.len() > 1 && ctod[*result.last().unwrap() as usize] == 0 {
                return false;
            }

            return true;
        }
        // the final left case
        if i == words.len() {
            // now a sum is final, we check right digit
            // already bind?
            if ctod[result[j] as usize] != -1 {
                // not match?
                if ctod[result[j] as usize] != sum % 10 {
                    return false;
                }
                // ok, process next pos
                return Self::dfs(words, result, ctod, dtoc, j + 1, 0, sum / 10);
            } else {
                // now we create a bind, but if the reverse direction bind is realdy exist
                // this is a invalid case
                if dtoc[(sum % 10) as usize] {
                    return false;
                }
                // create a bind
                dtoc[(sum % 10) as usize] = true;
                ctod[result[j] as usize] = sum % 10;

                if Self::dfs(words, result, ctod, dtoc, j + 1, 0, sum / 10) {
                    return true;
                }

                dtoc[(sum % 10) as usize] = false;
                ctod[result[j] as usize] = -1;
                return false;
            }
        }
        // we are now in normal process of left
        // jump the shorter left
        if j >= words[i].len() {
            return Self::dfs(words, result, ctod, dtoc, j, i + 1, sum);
        }
        let idx = words[i][j] as usize;
        // already bind?
        if ctod[idx] != -1 {
            // leading zero
            if words[i].len() > 1 && j == words.len() - 1 && ctod[idx] == 0 {
                return false;
            }
            return Self::dfs(words, result, ctod, dtoc, j, i + 1, sum + ctod[idx]);
        } else {
            // create bind
            for d in 0..=9 {
                if dtoc[d] {
                    continue;
                }

                // leading zero?
                if d == 0 && words[i].len() > 1 && j == words[i].len() - 1 {
                    continue;
                }
                ctod[idx] = d as i32;
                dtoc[d] = true;

                if Self::dfs(words, result, ctod, dtoc, j, i + 1, sum + d as i32) {
                    return true;
                }

                ctod[idx] = -1;
                dtoc[d] = false;
            }
            return false;
        }
    }
}

fn main() {}
