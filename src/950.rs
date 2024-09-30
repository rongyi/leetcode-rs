use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort_unstable_by(|a, b| b.cmp(a));
        let mut ret = VecDeque::new();

        for &card in deck.iter() {
            if !ret.is_empty() {
                if let Some(last) = ret.pop_back() {
                    ret.push_front(last);
                }
            }
            ret.push_front(card);
        }

        ret.into()
    }
}

fn main() {}
