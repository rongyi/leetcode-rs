struct Solution;

impl Solution {
    pub fn is_strictly_palindromic(n: i32) -> bool {
        for i in 2..=n - 2 {
            let num = Self::tobase(n, i);
            if !Self::is_valid(&num) {
                return false;
            }
        }
        true
    }

    fn is_valid(n: &[i32]) -> bool {
        let mut i = 0;
        let mut j = n.len() - 1;
        while i < j {
            if n[i] != n[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }

        true
    }
    fn tobase(mut n: i32, base: i32) -> Vec<i32> {
        let mut ret = Vec::new();
        while n > 0 {
            ret.push(n % base);
            n /= base;
        }

        ret
    }
}
fn main() {}
