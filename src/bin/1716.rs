#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        // Calculate complete weeks
        let complete_weeks = n / 7;
        // Calculate remaining days
        let remaining_days = n % 7;

        // For complete weeks:
        // First week: 1+2+3+4+5+6+7 = 28
        // Second week: 2+3+4+5+6+7+8 = 35 = 28 + 7
        // Third week: 3+4+5+6+7+8+9 = 42 = 35 + 7
        // So the formula is: 28 * complete_weeks + 7 * (0+1+2+...+(complete_weeks-1))

        // Sum of arithmetic sequence: sum = n(a1 + an)/2
        // For 0+1+2+...+(complete_weeks-1):
        // n = complete_weeks, a1 = 0, an = complete_weeks-1
        // sum = complete_weeks * (0 + complete_weeks-1) / 2
        // sum = complete_weeks * (complete_weeks-1) / 2

        let complete_weeks_money =
            28 * complete_weeks + 7 * (complete_weeks * (complete_weeks - 1) / 2);

        // For remaining days:
        // Start with (complete_weeks + 1) and add sequentially
        let mut remaining_days_money = 0;
        for day in 0..remaining_days {
            remaining_days_money += (complete_weeks + 1 + day);
        }

        complete_weeks_money + remaining_days_money
    }
}

fn main() {}
