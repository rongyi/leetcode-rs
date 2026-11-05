struct Solution;

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let text: Vec<&str> = text.split(' ').collect();
        let sz = text.len();
        let mut ret = Vec::new();
        for j in 1..sz - 1 {
            if text[j] == second && text[j - 1] == first {
                ret.push(text[j + 1].to_string());
            }
        }

        ret
    }
}

fn main() {}
