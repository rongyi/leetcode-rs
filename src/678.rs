struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut normal: Vec<usize> = Vec::new();
        let mut patch_stars: Vec<usize> = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                normal.push(i);
            } else if c == '*' {
                patch_stars.push(i);
            } else {
                if normal.is_empty() && patch_stars.is_empty() {
                    return false;
                }
                // consume ( first
                if !normal.is_empty() {
                    normal.pop();
                } else {
                    patch_stars.pop();
                }
            }
        }
        while !normal.is_empty() && !patch_stars.is_empty() {
            // *( case can not pass
            if *normal.last().unwrap() > *patch_stars.last().unwrap() {
                return false;
            }
            normal.pop();
            patch_stars.pop();
        }

        normal.is_empty()
    }
}

fn main() {}
