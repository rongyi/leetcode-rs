struct Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let ws: Vec<Vec<char>> = sentence.split(' ').map(|s| s.chars().collect()).collect();
        let first = ws.first().unwrap()[0];
        let last = *ws.last().unwrap().last().unwrap();
        if first != last {
            return false;
        }
        for i in 1..ws.len() {
            let prev_last = *ws[i - 1].last().unwrap();
            let cur_first = *ws[i].first().unwrap();
            if prev_last != cur_first {
                return false;
            }
        }

        true
    }
}

fn main() {}
