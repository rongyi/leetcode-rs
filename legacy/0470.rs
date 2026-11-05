struct Solution;

/**
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7
 * fn rand7() -> i32;
 */

impl Solution {
    pub fn rand10() -> i32 {
        let mut a = 7;
        let mut b = 7;
        while a == 7 {
            a = rand7();
        }
        while b > 5 {
            b = rand7();
        }
        // it has 1/2 probability to take another half
        if a < 4 {
            b += 5;
        }

        b
    }
}

fn main() {}
