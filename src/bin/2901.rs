struct Solution;

impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        fn hamming_distance(w1: &str, w2: &str) -> usize {
            w1.chars().zip(w2.chars()).filter(|&(a, b)| a != b).count()
        }
        let sz = words.len();

        let mut dp = vec![1; sz];
        let mut parent: Vec<i32> = vec![-1; sz];
        let mut max_len = 1;
        let mut last_idx = 0;
        for i in 0..sz {
            for j in 0..i {
                if groups[i] != groups[j] && words[i].len() == words[j].len() {
                    if hamming_distance(&words[i], &words[j]) == 1 {
                        if dp[j] + 1 > dp[i] {
                            dp[i] = dp[j] + 1;
                            parent[i] = j as i32;
                        }
                    }
                }
            }
            if dp[i] > max_len {
                max_len = dp[i];
                last_idx = i;
            }
        }
        let mut ret: Vec<String> = vec![];
        let mut cur = last_idx as i32;
        while cur != -1 {
            ret.push(words[cur as usize].clone());
            cur = parent[cur as usize];
        }

        ret.reverse();
        ret
    }
}

fn main() {}
