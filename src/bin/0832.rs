#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = image.len();
        let n = image[0].len();
        let mut ret = image.clone();
        for i in 0..m {
            for j in 0..n {
                // the original image
                ret[i][j] = if image[i][n - j - 1] == 1 { 0 } else { 1 };
            }
        }

        ret
    }
}

fn main() {}
