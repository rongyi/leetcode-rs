struct Solution;

impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let s: Vec<char> = answer_key.chars().collect();
        let twin = Self::sliding_win(&s, k, 'T');
        let fwin = Self::sliding_win(&s, k, 'F');

        twin.max(fwin)
    }
    fn sliding_win(s: &[char], k: i32, c: char) -> i32 {
        let sz = s.len();
        let mut cnt = 0;
        let mut ret = 0;
        let mut i = 0;
        for j in 0..sz {
            if s[j] == c {
                cnt += 1;
            }
            while cnt > k {
                if s[i] == c {
                    cnt -= 1;
                }
                i += 1;
            }
            ret = ret.max(j - i + 1);
        }

        ret as _
    }
}

fn main() {}
