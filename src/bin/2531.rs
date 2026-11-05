struct Solution;

impl Solution {
    pub fn is_it_possible(word1: String, word2: String) -> bool {
        let mut cnt1: Vec<i32> = vec![0; 26];
        let mut cnt2: Vec<i32> = vec![0; 26];
        for c in word1.chars() {
            let idx = (c as u8 - 'a' as u8) as usize;
            cnt1[idx] += 1;
        }

        for c in word2.chars() {
            let idx = (c as u8 - 'a' as u8) as usize;
            cnt2[idx] += 1;
        }
        let uniq1 = cnt1.iter().filter(|c| **c > 0).count();
        let uniq2 = cnt2.iter().filter(|c| **c > 0).count();
        for (i, &c1) in cnt1.iter().enumerate() {
            if c1 == 0 {
                continue;
            }

            for (j, &c2) in cnt2.iter().enumerate() {
                if c2 == 0 {
                    continue;
                }
                if i == j {
                    if uniq1 == uniq2 {
                        return true;
                    }
                    continue;
                }
                // swap i, j
                // for i value, if multiple current set dont change, or will shrink 1, for value wapped in if already existed, no change, else add 1
                let d1 = uniq1 - if c1 == 1 { 1 } else { 0 } + if cnt1[j] == 0 { 1 } else { 0 };
                // same
                let d2 = uniq2 - if c2 == 1 { 1 } else { 0 } + if cnt2[i] == 0 { 1 } else { 0 };

                if d1 == d2 {
                    return true;
                }
            }
        }

        false
    }
}

fn main() {}
