struct Solution;

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let mut uniq = 1;
        let mut s: Vec<char> = s.chars().collect();
        let n = s.len();
        // dp[i][j] count for [i, j] inclusive
        let mut dp = vec![vec![0; n]; n];

        for i in (0..n).rev() {
            // just one char
            dp[i][i] = 1;
            for j in i + 1..n {
                // if j and j - 1 char the same
                // then print them together
                if s[j] == s[j - 1] {
                    dp[i][j] = dp[i][j - 1];
                    continue;
                }
                // default case: just print new char on j index
                dp[i][j] = dp[i][j - 1] + 1;

                // 再来看一个字符串"abcb"，我们知道需要需要三步，我们看如果把这个字符串分成两个部分"a"和"bcb"，它们分别的步数是1和2，加起来的3是整个的步数。
                // 而对于字符串"abba"，如果分成"a"和"bba"，它们分别的步数也是1和2，但是总步数却是2。这是因为分出的"a"和"bba"中的最后一个字符相同。
                // 对于字符串"abbac"，因为位置0上的a和位置3上的a相同，那么整个字符串的步数相当于"bb"和"ac"的步数之和，为3。那么分析到这，是不是有点眉目了？
                // 我们关心的是字符相等的地方，对于[i, j]范围的字符，我们从i+1位置上的字符开始遍历到j，如果和i位置上的字符相等，我们就以此位置为界，
                // 将[i+1, j]范围内的字符拆为两个部分，将二者的dp值加起来，和原dp值相比，取较小的那个。所以我们的递推式如下:

                // dp[i][j] = min(dp[i][j], dp[i + 1][k - 1] + dp[k][j]       (s[k] == s[i] and i + 1 <= k <= j)
                // 要注意一些初始化的值，dp[i][i]是1，因为一个字符嘛，打印1次，还是就是在遍历k之前，
                // dp[i][j]初始化为 1 + dp[i + 1][j]，为啥呢，可以看成在[i + 1, j]的范围上多加了一个s[i]字符，
                // 最坏的情况就是加上的是一个不曾出现过的字符，步数顶多加1步，注意我们的i是从后往前遍历的，当然你可以从前往后遍历，参数对应好就行了，参见代码如下：
                for k in i..j {
                    if s[k] == s[j] {
                        dp[i][j] = dp[i][j].min(dp[i][k] + dp[k + 1][j - 1]);
                    }
                }
            }
        }

        dp[0][n - 1]
    }
}

fn main() {
    Solution::strange_printer("aaabbb".to_string());
}
