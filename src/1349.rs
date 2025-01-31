#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        let m = seats.len();
        let n = seats[0].len();
        let mut seat_mask = vec![0; m];
        for i in 0..m {
            for j in 0..n {
                // valid seat
                if seats[i][j] == '.' {
                    seat_mask[i] |= 1 << j;
                }
            }
        }
        let mut dp = vec![vec![0; 1 << n]; m + 1];
        for i in 1..=m {
            for prev_mask in 0..(1 << n) {
                // no '11' as neighbour
                if (prev_mask & (prev_mask << 1)) != 0 {
                    continue;
                }

                for cur_mask in 0..(1 << n) {
                    // same as prev_mask
                    if (cur_mask & (cur_mask << 1)) != 0 {
                        continue;
                    }
                    // cur mask every 1 has a valid seat, can sitdown
                    if (cur_mask & seat_mask[i - 1]) == cur_mask
                        && (cur_mask & (prev_mask >> 1)) == 0 // no upper left
                        /*no upper right */
                        && (cur_mask & (prev_mask << 1)) == 0
                    {
                        dp[i][cur_mask] = dp[i][cur_mask]
                            .max(dp[i - 1][prev_mask] + cur_mask.count_ones() as i32);
                    }
                }
            }
        }

        *dp[m].iter().max().unwrap()
    }
}

fn main() {}
