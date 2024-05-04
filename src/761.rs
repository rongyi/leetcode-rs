#![allow(dead_code)]
struct Solution;

// Logical Thinking
// If we regard 1, 0 in the definition of the special string as '(' and ')' separately,
// the problem is actually to get the string which is so-called valid parenthesis and meanwhile is the lexicographically largest.
// It is intuitive that we prefer deeper valid parenthesis to sit in front (deeper means the string surrounded with more pairs of parenthesis, e.g., '(())' is deeper than '()' ). We can achieve that by sorting them reversely.
// we go through S. Whenever the parentheses we met can be balanced (i.e., balance == 0), we construct valid parentheses -- by putting ( on the left boundary, )on the right boundary, and doing with the inner part following the same pattern.
impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let mut cnt = 0;
        let mut i = 0;
        let mut ret = Vec::new();
        for (j, c) in s.chars().enumerate() {
            if c == '1' {
                cnt += 1;
            } else {
                cnt -= 1;
            }
            if cnt == 0 {
                ret.push(format!(
                    "1{}0",
                    Self::make_largest_special(s[i + 1..j].to_string())
                ));
                i = j + 1;
            }
        }

        ret.sort_by(|a, b| b.cmp(a));
        ret.join("")
    }
}

fn main() {}
