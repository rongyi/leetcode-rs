struct Solution;

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut score = vec![0; 26];
        for (i, c) in order.chars().enumerate() {
            let idx = ((c as u8) - 'a' as u8) as usize;
            score[idx] = i;
        }

        let mut s: Vec<char> = s.chars().collect();

        s.sort_by(|&l, &r| {
            let lidx = (l as u8 - 'a' as u8) as usize;
            let ridx = (r as u8 - 'a' as u8) as usize;
            score[lidx].cmp(&score[ridx])
        });

        s.into_iter().collect()
    }
}

fn main() {}
