use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        // North, East, South, West
        let dirs = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        let mut x = 0;
        let mut y = 0;
        // 0 north
        // 1 east
        // 2 south
        // 3 west
        let mut direction = 0;
        let mut ret = 0;

        let obstacle_set: HashSet<(i32, i32)> =
            obstacles.into_iter().map(|obs| (obs[0], obs[1])).collect();

        for c in commands.into_iter() {
            if c == -2 {
                direction = (direction + 4 - 1) % 4;
            } else if c == -1 {
                direction = (direction + 1) % 4;
            } else {
                for _ in 0..c {
                    let nx = x + dirs[direction][0];
                    let ny = y + dirs[direction][1];
                    if !obstacle_set.contains(&(nx, ny)) {
                        x = nx;
                        y = ny;
                        ret = ret.max(x * x + y * y);
                    } else {
                        break;
                    }
                }
            }
        }

        ret
    }
}

fn main() {}
