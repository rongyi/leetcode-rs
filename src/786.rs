#![allow(dead_code)]

struct Solution;

use std::collections::BinaryHeap;

#[derive(PartialEq, Debug)]
struct Fraction {
    f: f64,
    numerator: usize,
    demoninator: usize,
}

impl Eq for Fraction {}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.f.partial_cmp(&self.f)
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f.partial_cmp(&self.f).unwrap()
    }
}

impl Solution {
    // [1, 7, 23, 29, 47]，那么有：
    // 1/47   < 1/29    < 1/23 < 1/7
    // 7/47   < 7/29    < 7/23
    // 23/47  < 23/29
    // 29/47
    // 规律： 每一列递增，每一行递增
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut pq: BinaryHeap<Fraction> = BinaryHeap::new();
        let sz = arr.len();
        for i in 0..sz - 1 {
            pq.push(Fraction {
                f: arr[i] as f64 / arr[sz - 1] as f64,
                numerator: i,
                demoninator: sz - 1,
            });
        }
        for _ in 0..k - 1 {
            let mut cur = pq.pop().unwrap();
            cur.demoninator -= 1;
            pq.push(Fraction {
                f: arr[cur.numerator] as f64 / arr[cur.demoninator] as f64,
                numerator: cur.numerator,
                demoninator: cur.demoninator,
            })
        }

        let cur = pq.pop().unwrap();
        vec![arr[cur.numerator], arr[cur.demoninator]]
    }
}

fn main() {}
