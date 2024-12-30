#![allow(dead_code)]


struct Solution;

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut start_box = (0, 0);
        let mut start_player = (0, 0);
        let mut target = (0, 0);

        for i in 0..m {
            for j in 0..n {
                match grid[i][j] {
                    'B' => start_box = (i, j),
                    'S' => start_player = (i, j),
                    'T' => target = (i, j),
                    _ => {}
                }
            }
        }
        // box_pos, palyer_pos, push_cnt
        let mut queue = VecDeque::new();
        // (box_pos, player_pos)
        let mut visited = HashSet::new();
        queue.push_back((start_box, start_player, 0));
        visited.insert((start_box, start_player));

        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        while let Some((box_pos, player_pos, push_cnt)) = queue.pop_front() {
            if box_pos == target {
                return push_cnt;
            }

            for &(dx, dy) in dirs.iter() {
                let new_box_x = box_pos.0 as i32 + dx;
                let new_box_y = box_pos.1 as i32 + dy;

                if new_box_x < 0
                    || new_box_x >= m as i32
                    || new_box_y < 0
                    || new_box_y >= n as i32
                    || grid[new_box_x as usize][new_box_y as usize] == '#'
                {
                    continue;
                }

                let required_player_x = box_pos.0 as i32 - dx;
                let required_player_y = box_pos.1 as i32 - dy;

                if required_player_x < 0
                    || required_player_x >= m as i32
                    || required_player_y < 0
                    || required_player_y >= n as i32
                    || grid[required_player_x as usize][required_player_y as usize] == '#'
                {
                    continue;
                }

                let required_pos = (required_player_x as usize, required_player_y as usize);

                if Self::can_reach(&grid, player_pos, required_pos, box_pos) {
                    let new_box_pos = (new_box_x as usize, new_box_y as usize);
                    let new_state = (new_box_pos, box_pos);
                    if !visited.contains(&new_state) {
                        visited.insert(new_state);
                        queue.push_back((new_box_pos, box_pos, push_cnt + 1));
                    }
                }
            }
        }

        -1
    }

    fn can_reach(
        grid: &Vec<Vec<char>>,
        start: (usize, usize),
        target: (usize, usize),
        box_pos: (usize, usize),
    ) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut q = VecDeque::new();
        q.push_back(start);
        visited[start.0][start.1] = true;
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        while let Some((x, y)) = q.pop_front() {
            if (x, y) == target {
                return true;
            }
            for &(dx, dy) in dirs.iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0
                    || nx >= m as i32
                    || ny < 0
                    || ny >= n as i32
                    || grid[nx as usize][ny as usize] == '#'
                    || (nx as usize, ny as usize) == box_pos
                    || visited[nx as usize][ny as usize]
                {
                    continue;
                }
                visited[nx as usize][ny as usize] = true;
                q.push_back((nx as usize, ny as usize));
            }
        }
        false
    }
}
fn main() {}
