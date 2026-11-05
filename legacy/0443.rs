struct Solution;

impl Solution {
    pub fn compress(raw: &mut Vec<char>) -> i32 {
        let sz = raw.len();
        let mut i = 0;
        let mut ret = String::new();
        while i < sz {
            let mut j = i;
            while j < sz && raw[j] == raw[i] {
                j += 1;
            }
            let repeat = j - i;
            ret.push(raw[i]);
            if repeat > 1 {
                ret.push_str(&repeat.to_string());
            }

            i = j;
        }

        let compressed: Vec<char> = ret.chars().collect();
        let csz = compressed.len();
        *raw = compressed;
        csz as i32
    }
}

fn main() {
    let mut input: Vec<char> = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    let a = Solution::compress(&mut input);
}
