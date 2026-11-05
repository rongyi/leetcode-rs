
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut heights = heights;
        heights.push(-1);

        let mut stack = Vec::new();
        let mut max_area = 0;

        for i in 0..heights.len() {
            while !stack.is_empty() && heights[i] < heights[*stack.last().unwrap()] {
                let height_index = stack.pop().unwrap();
                let width = if stack.is_empty() {
                    i as i32
                } else {
                    (i - stack.last().unwrap() - 1) as i32
                };

                let cur_area = width * heights[height_index];
                max_area = max_area.max(cur_area);
            }

            stack.push(i);
        }

        max_area
    }
}
