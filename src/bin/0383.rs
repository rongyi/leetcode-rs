struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut haystack = vec![0; 26];
        let mut needle = vec![0; 26];
        for c in ransom_note.chars() {
            needle[c as usize - 'a' as usize] += 1;
        }
        for c in magazine.chars() {
            haystack[c as usize - 'a' as usize] += 1;
        }
        needle.iter().zip(haystack.iter()).all(|p| p.0 <= p.1)
    }
}

fn main() {}
