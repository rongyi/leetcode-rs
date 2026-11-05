
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut heights = vec![0; n + 1];

        let mut ret = 0;
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
        let mut stack = Vec::new();

        let mut ret = 0;
        for i in 0..heights.len() {
            while !stack.is_empty() && heights[i] < heights[*stack.last().unwrap()] {
                let height_index = stack.pop().unwrap();
                let width = if stack.is_empty() {
                    i
                } else {
                    i - stack.last().unwrap() - 1
                };
                let cur = heights[height_index] * width as i32;
                ret = ret.max(cur);
            }
            stack.push(i);
        }
        ret
    }
}
