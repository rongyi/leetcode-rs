#![allow(dead_code)]


struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let origin_color = image[sr as usize][sc as usize];
        // do nothing
        if origin_color == color {
            return image;
        }
        let m = image.len() as i32;
        let n = image[0].len() as i32;

        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        q.push_back((sr, sc));
        while !q.is_empty() {
            let sz = q.len();
            for _ in 0..sz {
                let (x, y) = q.pop_front().unwrap();
                if image[x as usize][y as usize] != origin_color {
                    continue;
                }
                // update color
                image[x as usize][y as usize] = color;

                for d in [[0, 1], [1, 0], [-1, 0], [0, -1]].iter() {
                    let nx = x + d[0];
                    let ny = y + d[1];

                    if nx < 0
                        || nx >= m
                        || ny < 0
                        || ny >= n
                        || image[nx as usize][ny as usize] != origin_color
                    {
                        continue;
                    }
                    q.push_back((nx, ny));
                }
            }
        }

        image
    }
}

fn main() {}
