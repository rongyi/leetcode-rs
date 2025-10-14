struct Solution;

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let s: Vec<usize> = s.chars().map(|c| (c as u8 - 'a' as u8) as usize).collect();
        let sz = s.len();
        let mut acc = vec![0; 3];
        for &v in s.iter() {
            acc[v] += 1;
        }
        if acc.iter().any(|&c| c < k) {
            return -1;
        }
        let mut i = 0;
        let mut win_sz = 0;
        let mut ret = sz;
        // in win range we consume one, and left is decresing
        for j in 0..sz {
            // left for user to take, from beginning or end
            acc[s[j]] -= 1;
            win_sz += 1;
            while acc[s[j]] < k {
                acc[s[i]] += 1;
                i += 1;

                win_sz -= 1;
            }
            ret = ret.min(sz - win_sz);
        }

        ret as _
    }
}

fn main() {}
