struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort_unstable();
        let mut q = VecDeque::new();

        // to simulate
        for &card in deck.iter().rev() {
            if let Some(last) = q.pop_back() {
                q.push_front(last);
            }
            q.push_front(card);
        }

        q.into()
    }
}

fn main() {}
