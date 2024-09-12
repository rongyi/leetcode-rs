struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let mut ret = 0;
        let l: i64 = left.parse().unwrap();
        let r: i64 = right.parse().unwrap();

        Self::recur(VecDeque::new(), &mut ret, l, r);

        for c in '0'..='9' {
            let mut cur = VecDeque::new();
            cur.push_back(c);
            Self::recur(cur, &mut ret, l, r);
        }

        ret
    }
    fn recur(cur: VecDeque<char>, ret: &mut i32, l: i64, r: i64) {
        if cur.len() > 9 {
            return;
        }
        // ok, a valid
        if !cur.is_empty() && cur[0] != '0' {
            let cur_num: i64 = cur.iter().copied().collect::<String>().parse().unwrap();
            let cur_sq = cur_num * cur_num;
            if cur_sq > r {
                return;
            }
            let sqs = cur_sq.to_string();
            if cur_sq >= l && Self::is_palindrome(&sqs) {
                *ret += 1;
            }
        }

        for c in '0'..='9' {
            let mut next_cur = cur.clone();
            next_cur.push_front(c);
            next_cur.push_back(c);
            Self::recur(next_cur, ret, l, r);
        }
    }

    fn is_palindrome(s: &str) -> bool {
        s.chars().eq(s.chars().rev())
    }
}

fn main() {}
