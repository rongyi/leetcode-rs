struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let s: Vec<char> = expression.chars().collect();
        let mut ups: Vec<i32> = Vec::new();
        let mut downs: Vec<i32> = Vec::new();
        let mut is_neg = false;
        let mut i = 0;
        while i < s.len() {
            if s[i] == '-' {
                is_neg = true;
                i += 1;
            } else if s[i] == '+' {
                is_neg = false;
                i += 1;
            } else {
                let mut j = i;
                while j < s.len() && s[j].is_digit(10) {
                    j += 1;
                }
                let mut cur_num = s[i..j]
                    .iter()
                    .cloned()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                if is_neg {
                    cur_num = -cur_num;
                }
                ups.push(cur_num);

                // pass '/'
                j += 1;
                // mark i a new digit start
                i = j;

                while j < s.len() && s[j].is_digit(10) {
                    j += 1;
                }
                let cur_num = s[i..j]
                    .iter()
                    .cloned()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                downs.push(cur_num);
                i = j;
            }
        }
        let uniq_downs: HashSet<i32> = downs.iter().cloned().collect();
        let all_lcm = uniq_downs
            .into_iter()
            .fold(1, |acc, cur| Self::lcm(acc, cur));
        let mut sum = 0;
        for (i, &num) in ups.iter().enumerate() {
            sum += num * all_lcm / downs[i];
        }
        if sum % all_lcm == 0 {
            return format!("{}/1", sum / all_lcm);
        }
        let g = Self::gcd(sum.abs(), all_lcm);
        format!("{}/{}", sum / g, all_lcm / g)
    }

    fn gcd(mut x: i32, mut y: i32) -> i32 {
        let mut r;
        while y > 0 {
            r = x % y;
            x = y;
            y = r;
        }
        x
    }

    fn lcm(x: i32, y: i32) -> i32 {
        (x * y) / Self::gcd(x, y)
    }
}

fn main() {}
