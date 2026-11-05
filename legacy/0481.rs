struct Solution;

impl Solution {
    // shitty
    pub fn magical_string(n: i32) -> i32 {
        let mut s: Vec<char> = vec![' '];
        let mut x = 0;
        for i in 1..=n {
            if i >= s.len() as i32 {
                x = i;
            } else {
                x = (s[i as usize] as u8 - '0' as u8) as i32;
            }

            if i % 2 == 1 {
                let cur = vec!['1'; x as usize];
                s.extend(cur);
            } else {
                let cur = vec!['2'; x as usize];
                s.extend(cur);
            }
        }
        println!("{:?}", s);

        s.into_iter()
            .take((n + 1) as usize)
            .filter(|&c| c == '1')
            .count() as i32
    }
}

fn main() {}
