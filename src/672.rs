struct Solution;

impl Solution {
    // https://leetcode.com/problems/bulb-switcher-ii/discuss/107278/Very-easy-to-understand-C%2B%2B-with-detailed-explanation
    pub fn flip_lights(n: i32, m: i32) -> i32 {
        if m == 0 {
            return 1;
        }
        if n == 1 {
            if m >= 1 {
                return 2;
            }
        }
        if n == 2 {
            if m == 1 {
                return 3;
            }
            if m >= 2 {
                return 4;
            }
        }
        if n >= 3 {
            if m == 1 {
                return 4;
            }
            if m == 2 {
                return 7;
            }
            if m >= 3 {
                return 8;
            }
        }

        return 0;
    }
}

fn main() {}
