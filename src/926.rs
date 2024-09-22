struct Solution;

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let mut one_cnt = 0;
        let mut flip_cnt = 0;
        for c in s.chars() {
            if c == '0' {
                // 2. When '0' comes, things become a little bit complicated. There are two options for us:
                // flip the newly appended '0' to '1', after counter_flip flips for the original string;
                // or flip counter_one '1' in the original string to '0'. Hence, the result of the next
                // step of DP, in the '0' case, is std::min(counter_flip + 1, counter_one);.
                flip_cnt += 1;
            } else {
                // 1. When '1' comes, no more flip should be applied, since '1' is appended to the
                // tail of the original string.
                one_cnt += 1;
            }
            flip_cnt = flip_cnt.min(one_cnt);
        }

        flip_cnt
    }
}

fn main() {}
