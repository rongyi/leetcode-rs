struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let sz = bombs.len();
        let mut graph = vec![vec![]; sz];
        for i in 0..sz {
            let (x1, y1, r) = (bombs[i][0] as i64, bombs[i][1] as i64, bombs[i][2] as i64);
            let rr = r * r;
            // are other in my bomb range?
            // if yes, we can add edge from i -> j
            for j in 0..sz {
                if i == j {
                    continue;
                }
                let (x2, y2) = (bombs[j][0] as i64, bombs[j][1] as i64);
                let dx = x1 - x2;
                let dy = y1 - y2;
                if dx * dx + dy * dy <= rr {
                    // single edge, reverse may not ok
                    graph[i].push(j);
                }
            }
        }

        let mut max_val = 0;
        for i in 0..sz {
            max_val = max_val.max(Self::bfs(i, &graph, sz));
        }

        max_val
    }

    fn bfs(start: usize, graph: &Vec<Vec<usize>>, sz: usize) -> i32 {
        let mut ret = 0;
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(start);
        let mut visited = vec![false; sz];
        visited[start] = true;

        while let Some(cur) = q.pop_front() {
            ret += 1;
            for &next in graph[cur].iter() {
                if visited[next] {
                    continue;
                }
                q.push_back(next);
                visited[next] = true;
            }
        }

        ret
    }
}
fn main() {}
