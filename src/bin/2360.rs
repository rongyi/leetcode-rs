struct Solution;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let sz = edges.len();
        let mut memo: Vec<(i32, i32)> = vec![(-1, -1); sz];
        let mut ret = -1;
        let mut visited = vec![false; sz];

        for i in 0..sz {
            if visited[i] {
                continue;
            }
            visited[i] = true;
            let mut j = i as i32;
            let mut dist = 0;
            while j != -1 {
                visited[j as usize] = true;
                let (cur_dist, start) = memo[j as usize];
                if start == -1 {
                    memo[j as usize] = (dist, i as i32);
                    dist += 1;
                } else {
                    if start == i as i32 {
                        ret = ret.max(dist - cur_dist);
                    }
                    // already in a loop that start from other node
                    break;
                }

                j = edges[j as usize];
            }
        }
        ret
    }
}

fn main() {}
