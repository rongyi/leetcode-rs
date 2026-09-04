struct Solution;

mod ai {
    struct Solution;

    impl Solution {
        pub fn min_flips_mono_incr(s: String) -> i32 {
            let s = s.as_bytes();
            let n = s.len();

            let total_ones = s.iter().filter(|&&c| c == b'1').count();
            let total_zeros = n - total_ones;

            let mut min_flips = total_ones.min(total_zeros);
            let mut ones_seen = 0;
            let mut zeros_seen = 0;

            // Try split before each position
            for i in 0..=n {
                if i > 0 {
                    if s[i - 1] == b'1' {
                        ones_seen += 1;
                    } else {
                        zeros_seen += 1;
                    }
                }

                let right_zeros = total_zeros - zeros_seen;
                let flips = ones_seen + right_zeros;
                min_flips = min_flips.min(flips);
            }

            min_flips as i32
        }
    }
}

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
