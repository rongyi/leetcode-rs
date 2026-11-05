struct Solution;

impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        heights.push(-1);
        let mut mono = vec![];
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
