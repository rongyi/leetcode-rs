#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut ret = 0;
        for w in words.into_iter() {
            let cur: Vec<char> = w.chars().collect();
            if Self::strechy(&s, &cur) {
                ret += 1;
            }
        }

        ret
    }

    fn strechy(s: &Vec<char>, t: &Vec<char>) -> bool {
        let mut i = 0;
        let mut j = 0;
        let ssz = s.len();
        let tsz = t.len();

        while i < ssz && j < tsz {
            if s[i] != t[j] {
                return false;
            }
            let mut k = i;
            while k < ssz && s[k] == s[i] {
                k += 1;
            }
            let sames = k - i;

            let mut l = j;
            while l < tsz && t[l] == t[j] {
                l += 1;
            }
            let samet = l - j;

            if sames != samet {
                if samet > sames {
                    return false;
                }
                if sames < 3 {
                    return false;
                }
            }

            i = k;
            j = l;
        }

        i == ssz && j == tsz
    }
}

fn main() {
    let words: Vec<String> = vec!["zzyy".to_string()];
    let val = Solution::expressive_words("zzzzzyyyyy".to_string(), words);
    println!("{}", val);
}
