struct Solution;

impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        let (a, b) = (Self::parse_time(&event1[0]), Self::parse_time(&event1[1]));
        let (c, d) = (Self::parse_time(&event2[0]), Self::parse_time(&event2[1]));

        !((b < c) || (d < a))
    }

    fn parse_time(s: &str) -> i32 {
        let mut cks = s.split(':');
        let hour = cks.next().unwrap();
        let h = hour.trim_start().parse::<i32>().unwrap();
        let m = cks.next().unwrap();
        let minite = m.trim_start().parse::<i32>().unwrap();
        h * 60 + minite
    }
}

fn main() {}
