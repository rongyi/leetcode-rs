struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s = s.as_bytes();
        let mut cur = vec![];
        let mut ret = vec![];

        Self::backtrack(s, 0, &mut cur, &mut ret);
        ret
    }

    fn backtrack(s: &[u8], pos: usize, cur: &mut Vec<String>, out: &mut Vec<Vec<String>>) {
        if pos == s.len() {
            out.push(cur.clone());
            return;
        }
        for end in pos..s.len() {
            let next_chunk = &s[pos..end + 1];
            let next_s = String::from_utf8(next_chunk.to_vec()).unwrap();
            if Self::is_parlindrom(next_chunk) {
                cur.push(next_s);

                Self::backtrack(s, end + 1, cur, out);

                cur.pop();
            }
        }
    }

    fn is_parlindrom(s: &[u8]) -> bool {
        for (&i, &j) in s.iter().zip(s.iter().rev()) {
            if i != j {
                return false;
            }
        }
        true
    }
}
fn main() {}
