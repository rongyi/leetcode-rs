struct Solution;

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len();

        // Collect positions of 1s in both images
        let mut ones1 = Vec::new();
        let mut ones2 = Vec::new();

        for i in 0..n {
            for j in 0..n {
                if img1[i][j] == 1 {
                    ones1.push((i as i32, j as i32));
                }
                if img2[i][j] == 1 {
                    ones2.push((i as i32, j as i32));
                }
            }
        }

        // If either image has no 1s, overlap is 0
        if ones1.is_empty() || ones2.is_empty() {
            return 0;
        }

        // Count translation vectors
        use std::collections::HashMap;
        let mut translation_count: HashMap<(i32, i32), i32> = HashMap::new();

        // For each pair of 1s (one from img1, one from img2),
        // calculate the translation needed to align them
        for (i1, j1) in &ones1 {
            for (i2, j2) in &ones2 {
                // Translation vector: move img1's (i1,j1) to img2's (i2,j2)
                let trans = (i2 - i1, j2 - j1);
                *translation_count.entry(trans).or_insert(0) += 1;
            }
        }

        // Find the maximum frequency among translation vectors
        *translation_count.values().max().unwrap_or(&0)
    }
}

fn main() {}
