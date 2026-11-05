struct Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let sz = s.len();
        let mut left = 0;
        let mut right = sz as i32;
        let mut ret = Vec::new();
        let mut last = 0;
        for c in s.chars() {
            if c == 'I' {
                ret.push(left);
                left += 1;
                last = left;
            } else {
                ret.push(right);
                right -= 1;
                last = right;
            }
        }

        ret.push(last);

        ret
    }
}

fn main() {}
