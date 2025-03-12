#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 1 {
            return 0;
        }

        // Create edges with Manhattan distance
        let mut edges = Vec::new();
        for i in 0..n {
            for j in i + 1..n {
                let dist =
                    (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
                edges.push((i, j, dist));
            }
        }

        // Sort edges by distance
        edges.sort_by_key(|&(_, _, dist)| dist);

        // Kruskal's algorithm with Union-Find
        let mut parent: Vec<usize> = (0..n).collect();

        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }

        fn union(parent: &mut Vec<usize>, x: usize, y: usize) -> bool {
            let root_x = find(parent, x);
            let root_y = find(parent, y);

            if root_x == root_y {
                return false;
            }

            parent[root_x] = root_y;

            true
        }

        let mut total_cost = 0;
        let mut edges_used = 0;

        for (u, v, dist) in edges {
            if union(&mut parent, u, v) {
                total_cost += dist;
                edges_used += 1;

                // Early termination: MST has n-1 edges
                if edges_used == n - 1 {
                    break;
                }
            }
        }

        total_cost
    }
}

fn main() {}
