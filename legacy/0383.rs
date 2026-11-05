struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut dict = vec![0; 26];
        let mut note = vec![0; 26];
        for c in ransom_note.chars() {
            note[c as usize - 'a' as usize] += 1;
        }
        for c in magazine.chars() {
            dict[c as usize - 'a' as usize] += 1;
        }
        for i in 0..dict.len() {
            if note[i] > dict[i] {
                return false;
            }
        }

        true
    }
}


fn main() {

}
