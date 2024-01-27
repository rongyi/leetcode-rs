struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut ugly_numbers = vec![1; n as usize];

        let (mut i2, mut i3, mut i5) = (0, 0, 0);

        for i in 1..n as usize {
            let next_ugly = std::cmp::min(
                ugly_numbers[i2] * 2,
                std::cmp::min(ugly_numbers[i3] * 3, ugly_numbers[i5] * 5),
            );
            ugly_numbers[i] = next_ugly;
            if ugly_numbers[i2] * 2 == next_ugly {
                i2 += 1;
            }
            if ugly_numbers[i3] * 3 == next_ugly {
                i3 += 1;
            }
            if ugly_numbers[i5] * 5 == next_ugly {
                i5 += 1;
            }
        }

        ugly_numbers[n as usize - 1]
    }
}

fn main() {}
