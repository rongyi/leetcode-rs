#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let total_spaces = text.chars().filter(|&c| c == ' ').count();
        let words: Vec<&str> = text.split_whitespace().collect();

        if words.len() == 1 {
            // Only one word, all spaces go at the end
            return format!("{}{}", words[0], " ".repeat(total_spaces));
        }

        // Calculate spaces between words and extra spaces
        let spaces_between = total_spaces / (words.len() - 1);
        let extra_spaces = total_spaces % (words.len() - 1);

        // Build the result string
        let spaces = " ".repeat(spaces_between);
        let result = words.join(&spaces);

        // Add extra spaces at the end
        format!("{}{}", result, " ".repeat(extra_spaces))
    }
}
fn main() {}
