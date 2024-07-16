struct Solution;

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut s: Vec<char> = dominoes.chars().collect();
        let sz = s.len();
        let mut r = -1;

        for i in 0..sz {
            if s[i] == 'L' {
                if r == -1 {
                    let mut j = i as i32 - 1;
                    while j >= 0 && s[j as usize] == '.' {
                        s[j as usize] = 'L';
                        j -= 1;
                    }
                } else {
                    // R...L case
                    let mut start = r + 1;
                    let mut end = i as i32 - 1;
                    while start < end {
                        s[start as usize] = 'R';
                        s[end as usize] = 'L';
                        start += 1;
                        end -= 1;
                    }
                    r = -1;
                }
            } else if s[i] == 'R' {
                if r != -1 {
                    let mut j = i as i32 - 1;
                    while j > r && s[j as usize] == '.' {
                        s[j as usize] = 'R';
                        j -= 1;
                    }
                }
                r = i as i32;
            }
        }
        // R.. case
        if r != -1 {
            let mut j = r as usize + 1;
            while j < sz && s[j] == '.' {
                s[j] = 'R';
                j += 1;
            }
        }

        s.into_iter().collect()
    }
}

fn main() {}
