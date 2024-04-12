struct Solution;

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = img.len();
        let n = img[0].len();
        let mut ret = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                let mut sum = img[i][j];
                let mut cnt = 1;
                for &dx in &[-1, 0, 1] {
                    for &dy in &[-1, 0, 1] {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = i as i32 + dx;
                        let ny = j as i32 + dy;
                        if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                            continue;
                        }
                        sum += img[nx as usize][ny as usize];
                        cnt += 1;
                    }
                }
                ret[i][j] = sum / cnt;
            }
        }

        ret
    }
}

fn main() {
    let input = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    Solution::image_smoother(input);
}
