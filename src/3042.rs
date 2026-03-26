struct Solution;

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let sz = words.len();
        let mut count = 0;
        for i in 0..sz - 1 {
            for j in i + 1..sz {
                if words[j].starts_with(&words[i]) && words[j].ends_with(&words[i]) {
                    count += 1;
                }
            }
        }

        count
    }
}

fn main() {}
