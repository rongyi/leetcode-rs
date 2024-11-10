struct Solution;

impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let s1: Vec<char> = str1.chars().collect();
        let s2: Vec<char> = str2.chars().collect();
        let m = s1.len();
        let n = s2.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 0..m {
            for j in 0..n {
                if s1[i] == s2[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
                }
            }
        }

        let mut ret = Vec::new();
        let mut i = m;
        let mut j = n;
        while i > 0 && j > 0 {
            if s1[i - 1] == s2[j - 1] {
                ret.push(s1[i - 1]);
                i -= 1;
                j -= 1;
            } else if dp[i - 1][j] > dp[i][j - 1] {
                ret.push(s1[i - 1]);
                i -= 1;
            } else {
                ret.push(s2[j - 1]);
                j -= 1;
            }
        }
        while i > 0 {
            ret.push(s1[i - 1]);
            i -= 1;
        }
        while j > 0 {
            ret.push(s2[j - 1]);
            j -= 1;
        }
        ret.reverse();

        ret.into_iter().collect()
    }
    // pub fn shortest_common_supersequenceMLE(str1: String, str2: String) -> String {
    //     let s1: Vec<char> = str1.chars().collect();
    //     let s2: Vec<char> = str2.chars().collect();
    //     let mut ret: Vec<char> = Vec::new();
    //     let mut i = 0;
    //     let mut j = 0;

    //     for c in Self::lcs(&s1, &s2) {
    //         while i < s1.len() && s1[i] != c {
    //             ret.push(s1[i]);
    //             i += 1;
    //         }
    //         while j < s2.len() && s2[j] != c {
    //             ret.push(s2[j]);
    //             j += 1;
    //         }
    //         ret.push(c);
    //         i += 1;
    //         j += 1;
    //     }

    //     if i < s1.len() {
    //         let s1_left: Vec<char> = s1[i..].iter().copied().collect();
    //         ret.extend(s1_left);
    //     }

    //     if j < s2.len() {
    //         let s2_left: Vec<char> = s2[j..].iter().copied().collect();
    //         ret.extend(s2_left);
    //     }
    //     ret.into_iter().collect()
    // }

    fn lcs(a: &[char], b: &[char]) -> Vec<char> {
        let m = a.len();
        let n = b.len();
        let mut dp = vec![vec![Vec::new(); n + 1]; m + 1];

        for i in 0..m {
            for j in 0..n {
                if a[i] == b[j] {
                    let mut cur = dp[i][j].clone();
                    cur.push(a[i]);
                    dp[i + 1][j + 1] = cur;
                } else {
                    if dp[i + 1][j].len() > dp[i][j + 1].len() {
                        dp[i + 1][j + 1] = dp[i + 1][j].clone();
                    } else {
                        dp[i + 1][j + 1] = dp[i][j + 1].clone();
                    }
                }
            }
        }

        dp[m][n].clone()
    }
}

fn main() {
    let s1: Vec<char> = "abac".chars().collect();
    let s2: Vec<char> = "cab".chars().collect();
    let s = Solution::lcs(&s1, &s2);
    println!("{:?}", s);
    let s = Solution::shortest_common_supersequence("abac".to_string(), "cab".to_string());
    println!("{}", s);
}
