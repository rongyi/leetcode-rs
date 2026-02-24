struct Solution;

impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        fn alternative(words: &Vec<String>, groups: &Vec<i32>, mut prev: i32) -> Vec<String> {
            let mut ret: Vec<String> = vec![];
            for i in 0..words.len() {
                if groups[i] != prev {
                    prev = groups[i];
                    ret.push(words[i].clone());
                }
            }

            ret
        }

        let r1 = alternative(&words, &groups, 0);
        let r2 = alternative(&words, &groups, 1);
        if r1.len() >= r2.len() {
            r1
        } else {
            r2
        }
    }
}

fn main() {}
