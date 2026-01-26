struct Solution;

impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let s1 = str1.as_bytes();
        let s2 = str2.as_bytes();
        let n1 = s1.len();
        let n2 = s2.len();
        let mut i = 0;
        let mut j = 0;
        while i < n1 && j < n2 {
            let c1 = s1[i];
            let c2 = s2[j];

            if c1 == c2 || (c1 - b'a') == (c2 - b'a' + 26 - 1) % 26 {
                j += 1;
            }

            i += 1;
        }

        j == n2
    }
}

fn main() {}
