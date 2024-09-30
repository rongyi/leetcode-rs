struct Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut char_order = vec![0; 26];
        let words: Vec<Vec<char>> = words.into_iter().map(|s| s.chars().collect()).collect();

        for (i, c) in order.chars().enumerate() {
            char_order[(c as u8 - b'a') as usize] = i;
        }

        for i in 0..words.len() - 1 {
            if !Self::is_sorted(&words[i], &words[i + 1], &char_order) {
                return false;
            }
        }

        true
    }

    fn is_sorted(word1: &[char], word2: &[char], char_order: &Vec<usize>) -> bool {
        let mut i = 0;
        while i < word1.len() && i < word2.len() {
            let c1 = word1[i];
            let c2 = word2[i];
            if c1 != c2 {
                return char_order[(c1 as u8 - b'a') as usize]
                    < char_order[(c2 as u8 - b'a') as usize];
            }
            i += 1;
        }

        word1.len() <= word2.len()
    }
}

fn main() {}
