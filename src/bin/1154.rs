struct Solution;

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut prefix_sum = vec![0; 12];
        for i in 1..prefix_sum.len() {
            prefix_sum[i] = prefix_sum[i - 1] + days[i - 1];
        }
        let parts: Vec<i32> = date.split('-').map(|s| s.parse::<i32>().unwrap()).collect();
        let (y, m, d) = (parts[0], parts[1], parts[2]);
        let mut ret = prefix_sum[m as usize - 1] + d;
        if m > 2 && Self::is_leap_year(y) {
            ret += 1;
        }

        ret
    }

    fn is_leap_year(year: i32) -> bool {
        ((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0)
    }
}

fn main() {
    let s = "2019-02-10".to_string();
    let y = s[0..4].to_string();
    let m = s[5..7].trim_start_matches('0').to_string();
    let d = s[8..].trim_start_matches('0').to_string();
    println!("{y} {m} {d}");
    Solution::day_of_year(s);
}
