struct Solution;

impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut dictionary: Vec<Vec<char>> = dictionary
            .into_iter()
            .map(|s| s.chars().collect())
            .collect();
        dictionary.sort_by(|l, r| {
            if l.len() != r.len() {
                return r.len().cmp(&l.len());
            }
            l.cmp(&r)
        });
        for word in dictionary.iter() {
            if word.len() > s.len() {
                continue;
            }
            if Self::is_subseq(word, &s) {
                return word.iter().collect();
            }
        }
        "".to_string()
    }

    fn is_subseq(needle: &[char], haystack: &[char]) -> bool {
        let mut j = 0;
        for i in 0..haystack.len() {
            if haystack[i] == needle[j] {
                j += 1;
            }
            if j == needle.len() {
                return true;
            }
        }
        false
    }
}

fn main() {}
