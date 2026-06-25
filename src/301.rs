struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut out: HashSet<String> = HashSet::new();

        let mut cur = vec![];
        let (left_remove, right_remove) = Self::count_remove(&s);

        Self::dfs(
            s.as_bytes(),
            0,
            left_remove,
            right_remove,
            0,
            &mut cur,
            &mut out,
        );

        out.into_iter().collect()
    }

    fn dfs(
        s: &[u8],
        pos: usize,
        left_remove: i32,
        right_remove: i32,
        open: i32,
        cur: &mut Vec<u8>,
        out: &mut HashSet<String>,
    ) {
        if pos == s.len() {
            if left_remove == 0 && right_remove == 0 && open == 0 {
                let cur_s = String::from_utf8_lossy(cur).to_string();
                out.insert(cur_s);
            }
            return;
        }
        let sz = cur.len();
        // 1. ignore current
        if s[pos] == b'(' && left_remove > 0 {
            Self::dfs(s, pos + 1, left_remove - 1, right_remove, open, cur, out);
        } else if s[pos] == b')' && right_remove > 0 {
            Self::dfs(s, pos + 1, left_remove, right_remove - 1, open, cur, out);
        }

        // 2. keep current
        cur.push(s[pos]);
        if s[pos] == b'(' {
            Self::dfs(s, pos + 1, left_remove, right_remove, open + 1, cur, out);
        } else if s[pos] == b')' {
            // guard with matched
            if open > 0 {
                Self::dfs(s, pos + 1, left_remove, right_remove, open - 1, cur, out);
            }
        } else {
            // non () chars, just push
            Self::dfs(s, pos + 1, left_remove, right_remove, open, cur, out);
        }

        cur.truncate(sz);
    }

    fn count_remove(s: &str) -> (i32, i32) {
        let mut left_remove = 0;
        let mut right_remove = 0;
        for c in s.chars() {
            if c == '(' {
                left_remove += 1;
            } else if c == ')' {
                if left_remove > 0 {
                    left_remove -= 1;
                } else {
                    right_remove += 1;
                }
            }
        }

        (left_remove, right_remove)
    }
}

fn main() {}
