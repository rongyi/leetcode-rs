struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let m = heights.len() as i32;
        let n = heights[0].len() as i32;
        for i in 0..m {
            for j in 0..n {
                if Self::both(&heights, i, j) {
                    ret.push(vec![i, j]);
                }
            }
        }

        ret
    }

    fn both(matrix: &Vec<Vec<i32>>, x: i32, y: i32) -> bool {
        let mut c1: HashSet<i32> = HashSet::new();
        let mut c2: HashSet<i32> = HashSet::new();

        Self::pacific(matrix, x, y, &mut c1) && Self::atlantic(matrix, x, y, &mut c2)
    }

    fn pacific(matrix: &Vec<Vec<i32>>, x: i32, y: i32, cache: &mut HashSet<i32>) -> bool {
        let m = matrix.len() as i32;
        let n = matrix[0].len() as i32;
        cache.insert(x * n + y);
        if x == 0 || y == 0 {
            return true;
        }
        let dirs = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        for &d in &dirs {
            let nx = x + d[0];
            let ny = y + d[1];
            if nx < 0
                || nx >= m
                || ny < 0
                || ny >= n
                || cache.contains(&(nx * n + ny))
                || matrix[nx as usize][ny as usize] > matrix[x as usize][y as usize]
            {
                continue;
            }
            let check = Self::pacific(matrix, nx, ny, cache);
            if check {
                return true;
            }
        }

        false
    }

    fn atlantic(matrix: &Vec<Vec<i32>>, x: i32, y: i32, cache: &mut HashSet<i32>) -> bool {
        let m = matrix.len() as i32;
        let n = matrix[0].len() as i32;
        cache.insert(x * n + y);
        if x == m - 1 || y == n - 1 {
            return true;
        }
        let dirs = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        for &d in &dirs {
            let nx = x + d[0];
            let ny = y + d[1];
            if nx < 0
                || nx >= m
                || ny < 0
                || ny >= n
                || cache.contains(&(nx * n + ny))
                || matrix[nx as usize][ny as usize] > matrix[x as usize][y as usize]
            {
                continue;
            }
            let check = Self::atlantic(matrix, nx, ny, cache);
            if check {
                return true;
            }
        }

        false
    }
}

fn main() {}
