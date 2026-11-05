#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        let mut min_string = s.clone();

        queue.push_back(s.clone());
        visited.insert(s);

        while let Some(current) = queue.pop_front() {
            // Check if current string is lexicographically smaller
            if current < min_string {
                min_string = current.clone();
            }

            // Apply operation 1: Add a to all odd-indexed digits
            let operation1 = Self::add_to_odd_indices(&current, a);
            if !visited.contains(&operation1) {
                visited.insert(operation1.clone());
                queue.push_back(operation1);
            }

            // Apply operation 2: Rotate the string by b
            let operation2 = Self::rotate_string(&current, b);
            if !visited.contains(&operation2) {
                visited.insert(operation2.clone());
                queue.push_back(operation2);
            }
        }

        min_string
    }

    fn add_to_odd_indices(s: &str, a: i32) -> String {
        let mut chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() {
            if i % 2 == 1 {
                // Odd index
                let digit = chars[i].to_digit(10).unwrap();
                let new_digit = (digit as i32 + a) % 10;
                chars[i] = std::char::from_digit(new_digit as u32, 10).unwrap();
            }
        }

        chars.into_iter().collect()
    }

    fn rotate_string(s: &str, b: i32) -> String {
        let b = b as usize % s.len();
        let chars: Vec<char> = s.chars().collect();
        let rotated: String = chars[s.len() - b..].iter().collect::<String>()
            + &chars[..s.len() - b].iter().collect::<String>();

        rotated
    }
}
fn main() {}
