
struct Solution;

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut lower_count = vec![0; 26];
        let mut upper_count = vec![0; 26];
        for c in s.chars() {
            if c.is_ascii_lowercase() {
                let idx = (c as u8 - 'a' as u8) as usize;
                lower_count[idx] += 1;
            } else {
                let idx = (c as u8 - 'A' as u8) as usize;
                upper_count[idx] += 1;
            }
        }
        for i in (0..26).rev() {
            if lower_count[i] > 0 && upper_count[i] > 0 {
                return (('A' as u8 + i as u8) as char).to_string();
            }
        }

        return "".to_string();
    }
}
fn main() {}
