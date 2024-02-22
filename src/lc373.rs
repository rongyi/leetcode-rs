struct Solution;
use std::collections::BinaryHeap;

// show use how user defined compare for custom cmp
#[derive(Debug, PartialEq, Eq)]
struct MyPair(i32, i32);

impl Ord for MyPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0 + self.1).cmp(&(other.0 + other.1))
    }
}
impl PartialOrd for MyPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut pq = BinaryHeap::new();
        let mut ret = Vec::new();
        let k = k as usize;
        for i in 0..k.min(nums1.len()) {
            for j in 0..k.min(nums2.len()) {
                if pq.len() < k {
                    pq.push(MyPair(nums1[i], nums2[j]));
                } else {
                    let MyPair(a, b) = pq.peek().unwrap();
                    if nums1[i] + nums2[j] < *a + *b {
                        pq.push(MyPair(nums1[i], nums2[j]));
                        pq.pop();
                    }
                }
            }
        }
        while !pq.is_empty() {
            let MyPair(a, b) = pq.pop().unwrap();
            ret.push(vec![a, b]);
        }

        ret
    }
}

fn main() {
    let mut pq = BinaryHeap::new();
    pq.push(MyPair(1, 2));
    pq.push(MyPair(3, 4));
    pq.push(MyPair(1, 30));
    let MyPair(i, j) = pq.pop().unwrap();
    println!("{} {}", i, j);
}
