struct Solution;

impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let mut q: Vec<char> = word.chars().collect();
        let mut sz = q.len();
        let mut i = 0;
        let mut total_insert = 0;
        while i < sz {
            let mut win = 1;
            // 'ab' 'bc' 'ac'
            if i + 1 < sz && q[i + 1] > q[i] {
                win += 1;
                // perfect 'abc'
                if q[i] == 'a' && q[i + 1] == 'b' {
                    if i + 2 < sz && q[i + 2] == 'c' {
                        i += 3;
                    } else {
                        total_insert += 3 - win;
                        i += 2;
                    }
                } else {
                    total_insert += 3 - win;
                    i += 2;
                }
            } else {
                total_insert += 3 - win;
                i += 1;
            }
        }

        total_insert
    }
}

fn main() {}
