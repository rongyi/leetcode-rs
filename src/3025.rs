struct Solution;

impl Solution {
    pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        // Sort by x ascending, then by y descending
        points.sort_by(|a, b| {
            if a[0] != b[0] {
                a[0].cmp(&b[0])
            } else {
                b[1].cmp(&a[1])
            }
        });

        let mut count = 0;

        for i in 0..n {
            let y1 = points[i][1];
            // Use a very small number for the initial max_y
            let mut max_y_seen = i32::MIN;

            for j in i + 1..n {
                let y2 = points[j][1];

                // A is (points[i][0], y1), B is (points[j][0], y2)
                // Condition 1: A is upper-left (y1 >= y2)
                // Condition 2: No points inside (y2 > max_y_seen)
                if y2 <= y1 && y2 > max_y_seen {
                    count += 1;
                    max_y_seen = y2;
                }
            }
        }

        count
    }
}

fn main() {}
