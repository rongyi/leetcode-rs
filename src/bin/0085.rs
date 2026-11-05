struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut heights = vec![0; n + 1];

        let mut ret = 0;
        // top down, layer as height, and it just become the 84 problem
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '1' {
                    heights[j] += 1;
                } else {
                    heights[j] = 0;
                }
            }
            let cur = Self::max_area(heights.clone());
            ret = ret.max(cur);
        }
        ret
    }

    fn max_area(heights: Vec<i32>) -> i32 {
        let mut mono = Vec::new();

        let mut max_area = 0;
        for (i, &v) in heights.iter().enumerate() {
            while let Some(&prev_idx) = mono.last() {
                if v < heights[prev_idx] {
                    mono.pop();
                    let height = heights[prev_idx];
                    // we can makesure that current height is the lowest val from the next pos of prev mono idx (when empty, just from start)
                    let width = if mono.is_empty() {
                        i as i32
                    } else {
                        (i - 1 - *mono.last().unwrap()) as i32
                    };
                    let cur_are = height * width;
                    max_area = max_area.max(cur_are);
                } else {
                    break;
                }
            }

            mono.push(i);
        }

        max_area
    }
}

fn main() {}
