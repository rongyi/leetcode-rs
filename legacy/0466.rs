struct Solution;

impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let len1 = s1.len();
        let len2 = s2.len();
        let mut count1 = 0;
        let mut count2 = 0;
        let mut i = 0;
        let mut j = 0;

        // like subsequnce
        while count1 < n1 {
            if s1[i] == s2[j] {
                j += 1;
                if j == len2 {
                    j = 0;
                    // we don't care the n2 now, we just count the single party repeat count
                    // and divide the n2 in final answer
                    count2 += 1;
                }
            }
            i += 1;
            if i == len1 {
                i = 0;
                count1 += 1;
            }
        }

        count2 / n2
    }
}

fn main() {}
