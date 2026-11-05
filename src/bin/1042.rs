struct Solution;

impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ret = vec![0; n as usize];
        let mut graph = vec![vec![]; n as usize];

        for p in paths.iter() {
            graph[(p[0] - 1) as usize].push(p[1] - 1);
            graph[(p[1] - 1) as usize].push(p[0] - 1);
        }

        for i in 0..n as usize {
            let mut color = vec![false; 5];
            for &nei in graph[i].iter() {
                // here initial case 0 is not used
                color[ret[nei as usize] as usize] = true;
            }
            // 0 is not used
            for c in 1..5 {
                if !color[c] {
                    ret[i] = c as i32;
                    break;
                }
            }
        }

        ret
    }
}

fn main() {}
