#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let sz = s.len();
        let mut dp = vec![vec![0; sz]; sz];
        let prime: i64 = 1e9 as i64 + 7;

        // for 1
        for i in 0..sz {
            dp[i][i] = 1;
        }
        // For 2 characters :
        // 1. if they are same : "aa" ans : "a", "aa"
        // 2. if they are not same : "ab" ans : "a", "b"
        for i in 0..sz - 1 {
            dp[i][i + 1] = 2;
        }

        for l in 2..sz {
            for i in 0..sz - l {
                let j = i + l;

                if s[i] != s[j] {
                    // for query : "abc" first and last "a" and "c" are not same
                    // we check for ans from "ab" and "bc"
                    //                                          i->j-1   i+1->j
                    //                                          "a","b"  "b","c"
                    //                     and delete the common from both ranges ie "b" which is i+1->j-1
                    dp[i][j] = dp[i + 1][j] + dp[i][j - 1] - dp[i + 1][j - 1];
                } else {
                    // Since the edge characters are same :
                    //         a b a
                    //         i   j
                    // the total number of palindromic subsequence obtained from in bwtween characters can
                    // individually be bounded between characters at i and j
                    // As in this case palindromic sequence by in between character : "b" is 1 ie "b"
                    // which can be bounded like : "aba"
                    // so in total its prev "b" : 1*2 ie 2;
                    dp[i][j] = dp[i + 1][j - 1] * 2;

                    // we track the characters which are similar to boundary characters i and j
                    let mut low = i + 1;
                    let mut high = j - 1;
                    while low <= high && s[low] != s[j] {
                        low += 1;
                    }
                    while high >= low && s[high] != s[i] {
                        high -= 1;
                    }
                    if low == high {
                        // There is one character which is similar. let query be : "abaca"
                        //         a  b  a  c  a
                        //         i    l/h    j
                        // For this we add 1 for it to make a palindromic subsequence with either of
                        // edge characters i.e with i or j ie "aa"
                        dp[i][j] += 1;
                    } else if low < high {
                        // There are more than one characters in bwtween;
                        //         a   b   a   a   a   c   a
                        //         i       l       h       j
                        // We have to deduct the number of palindrome in between low and high
                        // as its already been calculated earlier @ dp[i][j]*2;
                        dp[i][j] -= dp[low + 1][high - 1];
                    } else {
                        // There are no characters in between which are identical
                        //         a   b   c   a
                        //         i   h   l   j
                        // we need to add 2 for the edge characters they can contribute in answers like
                        // "a" and "aa"
                        dp[i][j] += 2;
                    }
                }
                if dp[i][j] < 0 {
                    dp[i][j] = (dp[i][j] + prime) % prime;
                } else {
                    dp[i][j] %= prime;
                }
            }
        }

        dp[0][sz - 1] as i32
    }
}

fn main() {}
