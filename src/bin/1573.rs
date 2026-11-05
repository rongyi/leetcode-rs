#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_ways(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();

        // Count total number of 1's in the string
        let total_ones: usize = chars.iter().filter(|&&c| c == '1').count();

        // If no 1's, use combination formula
        if total_ones == 0 {
            return (((n as i64 - 1) * (n as i64 - 2) / 2) % MOD) as i32;
        }

        // If not divisible by 3, impossible to split evenly
        if total_ones % 3 != 0 {
            return 0;
        }

        let target = total_ones / 3;

        // Find positions where we get exactly target and 2*target 1's
        let mut first_cut_start = 0;
        let mut first_cut_end = 0;
        let mut second_cut_start = 0;
        let mut second_cut_end = 0;
        let mut count = 0;

        for i in 0..n {
            if chars[i] == '1' {
                count += 1;
            }

            if count == target && first_cut_start == 0 {
                first_cut_start = i + 1;
            }

            if count == target + 1 && first_cut_end == 0 {
                first_cut_end = i;
            }
            if count == 2 * target && second_cut_start == 0 {
                second_cut_start = i + 1;
            }
            if count == 2 * target + 1 && second_cut_end == 0 {
                second_cut_end = i;
                break;
            }
        }

        let first_ways = (first_cut_end - first_cut_start + 1) as i64;
        let second_ways = (second_cut_end - second_cut_start + 1) as i64;

        (first_ways * second_ways % MOD) as i32
    }
}

fn main() {}
