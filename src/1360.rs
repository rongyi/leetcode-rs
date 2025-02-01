#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        let d1 = Self::days_since_epoch(&date1);
        let d2 = Self::days_since_epoch(&date2);
        (d1 - d2).abs()
    }

    fn days_since_epoch(date: &str) -> i32 {
        let parts: Vec<i32> = date.split('-').map(|s| s.parse().unwrap()).collect();
        let (y, m, d) = (parts[0], parts[1], parts[2]);

        let mut total_days = 0;

        for y in 1971..y {
            total_days += if Self::is_leap_year(y) { 366 } else { 365 };
        }
        for m in 1..m {
            total_days += Self::days_in_month(y, m);
        }
        total_days += d;

        total_days
    }

    fn days_in_month(y: i32, m: i32) -> i32 {
        match m {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            2 => {
                if Self::is_leap_year(y) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    fn is_leap_year(y: i32) -> bool {
        (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
    }
}

fn main() {}
