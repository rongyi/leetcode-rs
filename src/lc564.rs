struct Solution;

impl Solution {
    pub fn nearest_palindromic(s: String) -> String {
        let num = s.parse::<i64>().unwrap();
        let sz = s.len();
        // the so called parlindrom root: e.g. 12321 -> 123
        // 123321 -> 123
        let half = s[0..(sz + 1) / 2].parse::<i64>().unwrap();
        let mut candidates = vec![10i64.pow(sz as u32) + 1, 10i64.pow(sz as u32 - 1) - 1];

        for &i in &[half - 1, half, half + 1] {
            let mut cur = i.to_string();
            let rev = cur.chars().rev().collect::<String>();
            // keep only one
            if sz % 2 != 0 {
                cur.pop();
            }
            cur.push_str(&rev);
            candidates.push(cur.parse().unwrap());
        }

        candidates
            .into_iter()
            .filter(|&c| c != num)
            .min_by_key(|&c| ((c - num).abs(), c))
            .unwrap()
            .to_string()
    }
}

fn main() {
    let a = [1, 2, 3];
    let val = a.iter().min_by_key(|&&c| (c * -1, c)).unwrap();
    println!("{}", val);
}
