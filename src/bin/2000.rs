struct Solution;

impl Solution {
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        properties.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let mut cur_max = i32::MIN;
        let mut count = 0;
        for prop in properties.iter().rev() {
            if prop[1] < cur_max {
                count += 1;
            }
            cur_max = cur_max.max(prop[1]);
        }

        count
    }
}

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        match word.find(ch) {
            Some(idx) => {
                let mut chars = word.chars().collect::<Vec<char>>();
                chars[..=idx].reverse();
                chars.into_iter().collect()
            }
            None => word,
        }
    }
}

fn main() {}
