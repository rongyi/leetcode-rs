struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = n;
        loop {
            slow = Self::next(slow);
            fast = Self::next(Self::next(fast));
            if slow == 1 || fast == 1 {
                return true;
            }
            if slow == fast {
                return false;
            }
        }
    }
    fn next(mut n: i32) -> i32 {
        let mut ret = 0;
        while n != 0 {
            let c = n % 10;
            ret += c * c;
            n /= 10;
        }
        ret
    }
}

fn main() {}
