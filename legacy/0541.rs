struct Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut s: Vec<char> = s.chars().collect();
        let mut i = 0;
        let k = k as usize;
        let sz = s.len();
        while i < sz {
            let end = (i + k).min(sz);
            let rev: Vec<char> = s[i..end].iter().rev().map(|x| x.to_owned()).collect();
            s[i..end].copy_from_slice(&rev);
            i += 2 * k;
        }
        s.into_iter().collect()
    }
}

fn main() {
    let s = "00".parse::<i32>().unwrap();
    println!("{}", s);
}
