struct Solution;

impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 {
            return 9;
        }
        let start: i64 = 10i32.pow((n - 1) as u32) as i64;
        let end: i64 = (10i32.pow(n as u32) - 1) as i64;

        let max_product = (end as i64) * (end as i64);
        let mut first_half = max_product / 10i32.pow(n as u32) as i64;

        while first_half >= start {
            let cur = Self::mkpar(first_half);
            let mut i = end;
            while i >= start && cur <= max_product && i * i >= cur {
                if cur % i == 0 {
                    return (cur % 1337) as i32;
                }
                i -= 1;
            }
            first_half -= 1;
        }

        -1
    }

    fn mkpar(first_half: i64) -> i64 {
        let mut s = first_half.to_string();
        let p: String = s.chars().rev().collect();
        s.push_str(&p);

        s.parse::<i64>().unwrap()
    }
}
fn main() {}
