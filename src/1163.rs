#![allow(dead_code)]
struct Solution;

impl Solution {
    // We use "j" to find a better starting index. If any is found, we use it to update "i"

    // 1."i" is the starting index of the first substring
    // 2."j" is the staring index of the second substring
    // 3."k" is related to substring.length() (eg. "k" is substring.length()-1)

    // Case 1 (s[i+k]==s[j+k]):
    // -> If s.substr(i,k+1)==s.substr(j,k+1), we keep increasing k.
    // Case 2 (s[i+k]<s[j+k]):
    // -> If the second substring is larger, we update i with max(i+k+1,j).
    //Since we can skip already matched things (The final answer is s.substr(i))
    // Case 3 (s[i+k]>s[j+k]):
    // -> If the second substring is smaller, we just change the starting index of the second string to j+k+1.
    //Because s[j]~s[j+k] must be less than s[i], otherwise "i" will be updated by "j". So the next possible candidate is "j+k+1".

    pub fn last_substring(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let sz = s.len();
        // largest lexiographical start
        let mut i = 0;
        // candicate start
        let mut j = 1;
        // len
        let mut k = 0;
        while j + k < sz {
            if s[i + k] == s[j + k] {
                k += 1;
            } else if s[i + k] < s[j + k] {
                i = i + k + 1;
                k = 0;
                if i >= j {
                    j = i + 1;
                }
            } else {
                j = j + k + 1;
                k = 0;
            }
        }
        s[i..].iter().collect()
    }
}

fn main() {}
