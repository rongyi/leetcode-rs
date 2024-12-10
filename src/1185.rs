#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let day_week_name = [
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
        ];
        // 1971 1.1 is friday
        let days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut prefixsum = vec![0; 13];
        for i in 1..13 {
            prefixsum[i] = prefixsum[i - 1] + days[i - 1];
        }
        let mut total_days = 4
            + day
            + if month > 2 && Self::is_leap_year(year) {
                1
            } else {
                0
            };
        for i in 1971..year {
            total_days += 365;
            if Self::is_leap_year(i) {
                total_days += 1;
            }
        }
        total_days += prefixsum[month as usize - 1];

        day_week_name[total_days as usize % 7].to_string()
    }

    fn is_leap_year(y: i32) -> bool {
        ((y % 4 == 0) && (y % 100) != 0) || (y % 400) == 0
    }
}

fn main() {}
