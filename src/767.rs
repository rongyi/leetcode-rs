struct Solution;

use std::collections::{BinaryHeap, HashMap};
impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let s: Vec<_> = s.chars().collect();
        let mut char_cnt: HashMap<char, i32> = HashMap::new();
        for &c in s.iter() {
            *char_cnt.entry(c).or_insert(0) += 1;
        }
        let mut pq = BinaryHeap::new();
        for (&k, &v) in char_cnt.iter() {
            if v > (s.len() + 1) as i32 / 2 {
                return "".to_string();
            }
            pq.push((v, k));
        }
        let mut ret = String::new();
        while pq.len() >= 2 {
            let (mut cnt1, c1) = pq.pop().unwrap();
            let (mut cnt2, c2) = pq.pop().unwrap();
            ret.push(c1);
            ret.push(c2);
            cnt1 -= 1;
            cnt2 -= 1;
            if cnt1 > 0 {
                pq.push((cnt1, c1));
            }
            if cnt2 > 0 {
                pq.push((cnt2, c2));
            }
        }
        if !pq.is_empty() {
            ret.push(pq.pop().unwrap().1);
        }

        ret
    }
}

mod ai {
    use std::collections::BinaryHeap;
    use std::collections::HashMap;

    impl Solution {
        pub fn reorganize_string(s: String) -> String {
            // Count frequency of each character
            let mut freq = HashMap::new();
            for c in s.chars() {
                *freq.entry(c).or_insert(0) += 1;
            }

            // Max heap: store (frequency, character)
            let mut heap = BinaryHeap::new();
            for (&c, &count) in freq.iter() {
                heap.push((count, c));
            }

            let mut result = String::new();
            let mut prev = None; // (frequency, character) of previously used char

            while let Some((mut count, c)) = heap.pop() {
                result.push(c);
                count -= 1;

                // If there was a previous character, push it back
                // again, when we find prev is some, we push it in
                if let Some(prev_char) = prev {
                    heap.push(prev_char);
                    prev = None;
                }

                // If this character still has remaining count, save it for next iteration
                if count > 0 {
                    // not in heap, but cache in this variable
                    prev = Some((count, c));
                }
            }

            // If we have a leftover character that couldn't be placed
            if let Some((_, c)) = prev {
                // Check if it would create adjacent duplicates
                if result.chars().last() == Some(c) {
                    return String::new();
                }
                result.push(c);
            }

            result
        }
    }
}

fn main() {}
