struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut ret = 0;
        // the most freq cnt in  current sliding window substring
        let mut cur_max = 0;
        let mut start = 0;
        let s: Vec<char> = s.chars().collect();
        let sz = s.len();
        let k = k as usize;

        // all upper case, so a vector is enough
        let mut cnt = vec![0; 26];

        for end in 0..sz {
            let idx = (s[end] as u8 - 'A' as u8) as usize;
            cnt[idx] += 1;
            cur_max = cur_max.max(cnt[idx]);
            // shrink
            while end - start + 1 - cur_max > k {
                let idx = (s[start] as u8 - 'A' as u8) as usize;
                cnt[idx] -= 1;
                start += 1;
            }
            ret = ret.max(end - start + 1);
        }

        ret as i32
    }
}

fn main() {}
