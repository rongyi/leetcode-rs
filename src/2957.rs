struct Solution;

impl Solution {
    pub fn remove_almost_equal_characters(word: String) -> i32 {
        let mut word: Vec<char> = word.chars().collect();
        let mut changes = 0;
        let n = word.len();

        for i in 1..n {
            if (word[i] as i32 - word[i - 1] as i32).abs() <= 1 {
                changes += 1;

                word[i] = '*'; // Mark as changed to avoid affecting next comparison
            }
        }
        changes
    }
}

fn main() {}
