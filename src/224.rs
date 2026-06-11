struct Solution;

impl Solution {
    // there's no * /, just flat +/
    pub fn calculate(s: String) -> i32 {
        // when should you push value to stack?
        // when meet a (, other than that just accumulate
        let mut stack: Vec<(i32, i32)> = vec![];
        let s: Vec<char> = s.chars().collect();
        let sz = s.len();
        let mut i = 0;

        let mut acc = 0;
        let mut sign = 1;

        while i < sz {
            match s[i] {
                '0'..='9' => {
                    let mut j = i;
                    let mut cur = 0;
                    while j < sz && s[j].is_digit(10) {
                        cur = cur * 10 + (s[j] as u8 - '0' as u8) as i32;
                        j += 1;
                    }
                    // because we have i + 1 again, so bring one more less step
                    i = j - 1;
                    acc += cur * sign;
                    sign = 1;
                }
                '-' => {
                    sign = -sign;
                }
                '(' => {
                    stack.push((acc, sign));
                    acc = 0;
                    sign = 1;
                }
                ')' => {
                    let (prev_acc, prev_sign) = stack.pop().unwrap();
                    acc = prev_acc + acc * prev_sign
                }
                _ => {}
            }

            i += 1;
        }

        acc
    }
}

fn main() {
    let val = Solution::calculate("- (3 + (4 + 5))".to_string());
    println!("{}", val);
}
