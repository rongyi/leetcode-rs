struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s: Vec<char> = s.chars().collect();
        let mut ret = Vec::new();
        let mut cur = Vec::new();

        Self::backtrack(&s, 0, &mut cur, &mut ret);

        ret
    }

    fn backtrack(s: &[char], start: usize, cur: &mut Vec<String>, ret: &mut Vec<Vec<String>>) {
        if start >= s.len() {
            ret.push(cur.clone());
            return;
        }

        for end in start..s.len() {
            if Self::is_parlindrom(s, start, end) {
                let chunk: String = s[start..=end].iter().collect();

                cur.push(chunk);
                Self::backtrack(s, end + 1, cur, ret);
                cur.pop();
            }
        }
    }

    fn is_parlindrom(s: &[char], i: usize, j: usize) -> bool {
        let mut i = i;
        let mut j = j;
        while i < j {
            if s[i] != s[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

fn main() {}
