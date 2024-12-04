#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let mut words: Vec<i32> = words.into_iter().map(|s| Self::freq(&s)).collect();
        words.sort_unstable();

        let mut ret = Vec::new();

        for q in queries.into_iter() {
            let target = Self::freq(&q);
            let val = words.len() - Self::upper_bound(&words, target);
            ret.push(val as i32);
        }

        ret
    }

    fn upper_bound(lst: &Vec<i32>, target: i32) -> usize {
        let mut l = 0;
        let mut r = lst.len();
        while l < r {
            let mid = l + (r - l) / 2;
            if lst[mid] <= target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l
    }

    fn freq(s: &str) -> i32 {
        let c = s.chars().min().unwrap();
        s.chars().filter(|&x| x == c).count() as i32
    }
}

fn main() {}
