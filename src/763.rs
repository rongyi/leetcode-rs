#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let s: Vec<_> = s.chars().collect();
        let sz = s.len();
        let mut marks = Vec::new();
        let mut right_most = vec![0; 26];

        let mut next_index = 0;
        for (i, &c) in s.iter().enumerate() {
            let idx = (c as u8 - 'a' as u8) as usize;
            right_most[idx] = i;
        }

        while next_index < sz {
            let idx = (s[next_index] as u8 - 'a' as u8) as usize;
            let mut cur_right_most = right_most[idx];
            let mut i = next_index;

            // fetch right most
            while i <= cur_right_most {
                cur_right_most = cur_right_most.max(right_most[(s[i] as u8 - 'a' as u8) as usize]);
                i += 1;
            }
            marks.push(cur_right_most as i32 + 1);
            next_index = cur_right_most + 1;
        }

        let mut ret = marks.clone();
        for i in 1..ret.len() {
            ret[i] = marks[i] - marks[i - 1];
        }

        ret
    }
}

fn main() {
    let a = Solution::partition_labels("ababcbacadefegdehijhklij".to_string());
    println!("{:?}", a);
}
