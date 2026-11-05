struct Solution;

impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let idx: Vec<usize> = number
            .chars()
            .enumerate()
            .filter(|c| c.1 == digit)
            .map(|c| c.0)
            .collect();
        let mut delete_result: Vec<String> = Vec::new();
        for &id in idx.iter() {
            let mut cur = number.clone();
            cur.remove(id);
            delete_result.push(cur);
        }
        delete_result.sort_unstable();
        delete_result.pop().unwrap()
    }
}

fn main() {}
