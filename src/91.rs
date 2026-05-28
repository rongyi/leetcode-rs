struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut memo = vec![-1; s.len()];

        Self::helper(s, 0, &mut memo)
    }
    fn helper(s: &[u8], pos: usize, memo: &mut Vec<i32>) -> i32 {
        // find a valid decode way
        if pos >= s.len() {
            return 1;
        }
        if memo[pos] != -1 {
            return memo[pos];
        }
        // no way for leading zero
        if s[pos] == b'0' {
            memo[pos] = 0;
            return 0;
        }
        let w1 = Self::helper(s, pos + 1, memo);
        let mut w2 = 0;

        if pos + 1 < s.len() {
            let check = (s[pos] - b'0') * 10 + s[pos + 1] - b'0';
            if check >= 10 && check <= 26 {
                w2 = Self::helper(s, pos + 2, memo);
            }
        }
        memo[pos] = w1 + w2;

        w1 + w2
    }
}

fn main() {}
