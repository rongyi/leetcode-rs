struct Solution;

use std::{collections::HashMap, i32};
impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for e in edges.iter() {
            let (from, to) = (e[0], e[1]);
            graph.entry(from).or_default().push(to);
            graph.entry(to).or_default().push(from);
        }
        let sz = nums.len();
        let mut in_out_tag: Vec<(usize, usize)> = vec![(0, 0); sz];
        let mut xor: Vec<i32> = vec![0; sz];
        let mut timestamp = 0;
        Self::dfs(
            0,
            -1,
            &nums,
            &graph,
            &mut timestamp,
            &mut in_out_tag,
            &mut xor,
        );
        let total = xor[0];
        let mut ret: i32 = i32::MAX;

        for i in 1..sz {
            let (in_i, out_i) = in_out_tag[i];
            for j in i + 1..sz {
                let (in_j, out_j) = in_out_tag[j];
                let (a, b, c) = if in_i < in_j && out_j < out_i {
                    (xor[j], xor[i] ^ xor[j], total ^ xor[i])
                } else if in_j < in_i && out_i < out_j {
                    (xor[i], xor[j] ^ xor[i], total ^ xor[j])
                } else {
                    (xor[i], xor[j], total ^ xor[i] ^ xor[j])
                };
                let max = a.max(b).max(c);
                let min = a.min(b).min(c);
                ret = ret.min(max - min);
                if ret == 0 {
                    return 0;
                }
            }
        }

        ret
    }
    fn dfs(
        cur: i32,
        parent: i32,
        nums: &Vec<i32>,
        graph: &HashMap<i32, Vec<i32>>,
        timestamp: &mut usize,
        in_out_tag: &mut Vec<(usize, usize)>,
        xor: &mut Vec<i32>,
    ) {
        *timestamp += 1;
        in_out_tag[cur as usize].0 = *timestamp;

        xor[cur as usize] = nums[cur as usize];
        if let Some(neibs) = graph.get(&cur) {
            for &neib in neibs.iter() {
                if neib == parent {
                    continue;
                }
                Self::dfs(neib, cur, nums, graph, timestamp, in_out_tag, xor);
                xor[cur as usize] ^= xor[neib as usize];
            }
        }

        *timestamp += 1;
        in_out_tag[cur as usize].1 = *timestamp;
    }
}

fn main() {}
