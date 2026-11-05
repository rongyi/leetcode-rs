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
                break;
            }
        }

        false
    }
    fn next(mut n: i32) -> i32 {
        let mut sum = 0;

        while n != 0 {
            let v = n % 10;
            sum += v * v;
            n /= 10;
        }

        sum
    }
}

fn main() {
    Solution::is_happy(19);
}
