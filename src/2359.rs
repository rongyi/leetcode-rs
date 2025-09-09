
struct Solution;

use std::i32;
impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let sz = edges.len();
        let mut route1 = vec![-1; sz];
        let mut route2 = vec![-1; sz];
        Self::travel(node1, 0, &edges, &mut route1);
        Self::travel(node2, 0, &edges, &mut route2);

        let mut min_max_dist = i32::MAX;
        let mut ret = -1i32;
        for i in 0..sz {
            if route1[i] >= 0 && route2[i] >= 0 && route1[i].max(route2[i]) < min_max_dist {
                min_max_dist = route1[i].max(route2[i]);
                ret = i as _;
            }
        }
        ret
    }
    fn travel(mut cur: i32, mut depth_acc: i32, edges: &Vec<i32>, nodes_depth: &mut Vec<i32>) {
        while cur != -1 && nodes_depth[cur as usize] == -1 {
            nodes_depth[cur as usize] = depth_acc;
            depth_acc += 1;
            cur = edges[cur as usize];
        }
    }
}

fn main() {}
