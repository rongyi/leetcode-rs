struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut pq: BinaryHeap<Reverse<char>> = BinaryHeap::new();
        let mut s: Vec<char> = n.to_string().chars().collect();
        let origin = s.clone();
        // from bigger to small
        s.sort_by(|&l, &r| r.cmp(&l));
        // original order is mono decrese, e.g. 321
        if s == origin {
            return -1;
        }
        s = origin;
        let mut i = s.len() - 1;
        while i > 0 {
            pq.push(Reverse(s[i]));
            if s[i] <= s[i - 1] {
                i -= 1;
            } else {
                // find a one which start decrese
                // e.g.      4521
                //            ^
                //  we find the smallest number which is bigger than 4
                while !pq.is_empty() {
                    let Reverse(top) = *pq.peek().unwrap();
                    if top <= s[i - 1] {
                        pq.pop();
                    } else {
                        break;
                    }
                }
                let Reverse(d) = pq.pop().unwrap();
                let mut j = s.len() - 1;
                while s[j] != d {
                    j -= 1;
                }
                s.swap(i - 1, j);
                let mut sorted: Vec<char> = s[i..].iter().map(|c| c.to_owned()).collect();
                sorted.sort();
                s[i..].copy_from_slice(&sorted);
                break;
            }
        }
        let val: i64 = s.iter().collect::<String>().parse::<i64>().unwrap();
        if val > i32::MAX as i64 {
            return -1;
        }

        val as i32
    }
}

fn main() {
    let val = Solution::next_greater_element(1234);
    println!("{}", val);
}
