struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        fn is_vowel(c: char) -> bool {
            c == 'a'
                || c == 'e'
                || c == 'i'
                || c == 'o'
                || c == 'u'
                || c == 'A'
                || c == 'E'
                || c == 'I'
                || c == 'O'
                || c == 'U'
        }
        let mut s: Vec<char> = s.chars().collect();
        let sz = s.len();
        let (mut i, mut j) = (0, (sz - 1) as i32);
        while i < j {
            while (i as usize) < j as usize && !is_vowel(s[i as usize]) {
                i += 1;
            }
            while i < j && !is_vowel(s[j as usize]) {
                j -= 1;
            }
            s.swap(i as usize, j as usize);
            i += 1;
            j -= 1;
        }

        s.into_iter().collect()
    }
}

fn main() {}
