#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
        // I like m * n
        let (m, n) = (n as usize, m as usize);
        let mut height = vec![0; n];
        let mut min_squares = m * n;

        fn find_min_height(height: &Vec<usize>) -> usize {
            *height.iter().min().unwrap()
        }

        // take height h we find all height from start should be h, so we can put some rectangle
        fn find_width(height: &Vec<usize>, start: usize, h: usize) -> usize {
            let mut width = 0;
            for i in start..height.len() {
                if height[i] != h {
                    break;
                }
                width += 1;
            }
            width
        }

        fn backtrack(height: &mut Vec<usize>, count: usize, min_squares: &mut usize, n: usize) {
            if count >= *min_squares {
                return;
            }
            if height.iter().all(|&h| h == n) {
                *min_squares = count;
                return;
            }
            let h = find_min_height(height);
            let start = height.iter().position(|&x| x == h).unwrap();
            let max_width = find_width(height, start, h);
            let max_height = n - h;
            let max_size = max_width.min(max_height);

            for size in (1..=max_size).rev() {
                let can_place = (start..start + size).all(|i| height[i] == h);
                if !can_place {
                    continue;
                }
                for i in start..start + size {
                    height[i] += size;
                }
                backtrack(height, count + 1, min_squares, n);

                for i in start..start + size {
                    height[i] -= size;
                }
            }
        }

        backtrack(&mut height, 0, &mut min_squares, m);

        min_squares as i32
    }
}
fn main() {}
