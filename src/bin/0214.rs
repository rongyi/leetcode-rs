struct Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let s_rev: String = s.chars().rev().collect();
        let combined: String = format!("{}#{}", s, s_rev);
        let sz = combined.len();
        let mut lps = vec![0; sz];
        let mut j = 0;
        let cb = combined.as_bytes();
        for i in 1..sz {
            // j is len, so cb[j] is next try check var, cb[j-1] is current match len
            while j > 0 && cb[j] != cb[i] {
                j = lps[j - 1];
            }
            if cb[i] == cb[j] {
                j += 1;
            }
            lps[i] = j;
        }
        let saved_len = lps[sz - 1];
        // those are the added str in reverse order
        let mut ret: String = s[saved_len..].chars().rev().collect();

        // and append last to it
        ret.push_str(&s);
        ret
    }
}

fn main() {}
