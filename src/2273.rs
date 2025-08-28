struct Solution;

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut prev_sorted = vec![];
        let mut ret = vec![];
        ret.push(words[0].clone());
        prev_sorted.push(Self::origin(&words[0]));

        for i in 1..words.len() {
            if Self::origin(&words[i]) == *prev_sorted.last().unwrap() {
                continue;
            }
            ret.push(words[i].clone());
            prev_sorted.push(Self::origin(&words[i]));
        }

        ret
    }

    fn origin(s: &str) -> String {
        let mut cs: Vec<u8> = s.bytes().collect();
        cs.sort();
        String::from_utf8(cs).unwrap()
    }
}
fn main() {}
