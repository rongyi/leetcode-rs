struct Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut s: String = s.split('-').collect();
        s = s.to_uppercase();
        let s: Vec<char> = s.chars().collect();
        let mut i: usize = 0;
        let k = k as usize;

        let mut ret: Vec<Vec<char>> = Vec::new();
        // first chunk
        if s.len() % k != 0 {
            let first_chunk: Vec<char> = s[0..s.len() % k].iter().map(|c| c.to_owned()).collect();
            ret.push(first_chunk);
        }
        i = s.len() % k;
        while i < s.len() {
            let cur = s[i..i + k].iter().map(|c| c.to_owned()).collect();
            ret.push(cur);

            i += k;
        }

        let ret: Vec<String> = ret.into_iter().map(|v| v.into_iter().collect()).collect();
        ret.join("-")
    }
}

fn main() {
    Solution::license_key_formatting("2-5g-3-J".to_string(), 2);
}
