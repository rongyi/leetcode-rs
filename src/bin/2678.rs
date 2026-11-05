struct Solution;

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .into_iter()
            .map(|s| {
                let s = s.as_bytes();
                let s = str::from_utf8(&s[11..13]).unwrap();
                s.parse::<i32>().unwrap()
            })
            .filter(|v| *v > 60)
            .count() as _
    }
}

fn main() {}
