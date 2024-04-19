struct Solution;

impl Solution {
    // dp[i] = dp[i-1]* f(s.substr(i,1)) + dp[i-2]* f(s.substr(i-1, 2))
    pub fn num_decodings(s: String) -> i32 {
        let n = s.len();
        let m: i64 = 1e9 as i64 + 7;
        let mut f1 = 1i64;
        let s: Vec<char> = s.chars().collect();
        let mut f2 = Self::count(s.iter().take(1).cloned().collect());

        for i in 1..n {
            let f3 = f2 * Self::count(s[i..i + 1].iter().cloned().collect())
                + f1 * Self::count(s[i - 1..i + 1].iter().cloned().collect());
            f1 = f2;
            f2 = f3 % m;
        }

        f2 as i32
    }

    fn count(s: Vec<char>) -> i64 {
        // single char case
        if s.len() == 1 {
            if s[0] == '*' {
                return 9;
            }
            if s[0] == '0' {
                return 0;
            } else {
                return 1;
            }
        }
        // two char case
        // 11 12 ... 19 21 ... 26
        // 20 is not valid, because * can only be 1 ~ 9
        if s.iter().collect::<String>() == "**" {
            return 15;
        } else if s[1] == '*' {
            // second is  * case
            if s[0] == '1' {
                return 9;
            } else if s[0] == '2' {
                return 6;
            }
            // invalid
            return 0;
        } else if s[0] == '*' {
            // can be 1, 2
            if s[1] <= '6' {
                return 2;
            }
            // else only 1
            return 1;
        }
        let cur = s.iter().collect::<String>().parse::<i64>().unwrap();
        if cur >= 10 && cur <= 26 {
            return 1;
        }
        return 0;
    }
}

fn main() {
    let a = "hello";
    println!("{}", a[0]);
}
