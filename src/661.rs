struct Solution;

impl Solution {
    /// LeetCode 661: Image Smoother
    ///
    /// Each cell becomes the floor of the average of itself and its
    /// (up to 8) in-bounds neighbors.
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (img.len(), img[0].len());
        let mut ret = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                let mut sum = img[i][j];
                let mut cnt = 1;

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue; // self already counted
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

                ret[i][j] = sum / cnt; // floor for non-negative values
            }
        }

        ret
    }
}

fn main() {
    let tests = [
        (
            vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
        ),
        (
            vec![vec![100, 200, 100], vec![200, 50, 200], vec![100, 200, 100]],
            vec![
                vec![137, 141, 137],
                vec![141, 138, 141],
                vec![137, 141, 137],
            ],
        ),
    ];

    for (img, expected) in &tests {
        let result = Solution::image_smoother(img.clone());
        println!(
            "{} img={:?}\n   → {:?} (expected {:?})",
            if result == *expected { "✓" } else { "✗" },
            img,
            result,
            expected
        );
    }
}
