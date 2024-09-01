struct Solution;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let cnt = Self::count_digit(n);

        (0..31).any(|i| Self::count_digit(1 << i) == cnt)
    }

    fn count_digit(mut n: i32) -> [u8; 10] {
        let mut count = [0; 10];
        while n > 0 {
            count[(n % 10) as usize] += 1;
            n /= 10;
        }

        count
    }
}

fn main() {}
