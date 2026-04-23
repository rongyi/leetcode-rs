struct Solution;

impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let n = bottom_left.len();
        let mut max_square_area: i64 = 0;

        for i in 0..n {
            for j in i + 1..n {
                // Determine the boundaries of the intersection
                let inter_left = bottom_left[i][0].max(bottom_left[j][0]);
                let inter_right = top_right[i][0].min(top_right[j][0]);
                let inter_bottom = bottom_left[i][1].max(bottom_left[j][1]);
                let inter_top = top_right[i][1].min(top_right[j][1]);

                // Check if there is a valid intersection
                if inter_right > inter_left && inter_top > inter_bottom {
                    let width = (inter_right - inter_left) as i64;
                    let height = (inter_top - inter_bottom) as i64;

                    // The largest square in this intersection has side = min(width, height)
                    let side = width.min(height);
                    let area = side * side;

                    if area > max_square_area {
                        max_square_area = area;
                    }
                }
            }
        }

        max_square_area
    }
}

fn main() {}
