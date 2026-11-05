struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut ret = Vec::new();
        let mut q: VecDeque<(String, usize)> = VecDeque::new();
        q.push_back((s, 0));

        while !q.is_empty() {
            let (cur, idx) = q.pop_front().unwrap();

            if Self::is_valid(&cur) {
                ret.push(cur);
            } else if ret.is_empty() {
                for i in idx..cur.len() {
                    let c = cur.chars().nth(i).unwrap();
                    if (c == ')' || c == '(') && (i == idx || c != cur.chars().nth(i - 1).unwrap())
                    {
                        q.push_back((
                            format!("{}{}", cur.get(0..i).unwrap(), cur.get(i + 1..).unwrap()),
                            i,
                        ));
                    }
                }
            }
        }

        ret
    }

    fn is_valid(s: &str) -> bool {
        let mut cnt = 0;
        for c in s.chars() {
            if c == '(' {
                cnt += 1;
            }
            if c == ')' {
                if cnt > 0 {
                    cnt -= 1;
                } else {
                    return false;
                }
            }
        }

        cnt == 0
    }
}
fn main() {
    println!("Hello, world!");
}
