#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn reformat_date(date: String) -> String {
        let parts: Vec<&str> = date.split(' ').collect();
        let day: i32 = parts[0]
            .chars()
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();
        let month = match parts[1] {
            "Jan" => "01",
            "Feb" => "02",
            "Mar" => "03",
            "Apr" => "04",
            "May" => "05",
            "Jun" => "06",
            "Jul" => "07",
            "Aug" => "08",
            "Sep" => "09",
            "Oct" => "10",
            "Nov" => "11",
            "Dec" => "12",
            _ => unreachable!(),
        };
        format!("{}-{}-{:02}", parts[2], month, day)
    }
}

fn main() {}
