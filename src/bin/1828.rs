#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::with_capacity(queries.len());

        for query in queries {
            let center_x = query[0];
            let center_y = query[1];
            let radius = query[2];
            let radius_squared = radius * radius;

            let mut count = 0;
            for point in &points {
                let x = point[0];
                let y = point[1];

                // Calculate squared distance from point to center
                let distance_squared = (x - center_x).pow(2) + (y - center_y).pow(2);

                // If the distance is less than or equal to the radius, point is inside circle
                if distance_squared <= radius_squared {
                    count += 1;
                }
            }

            result.push(count);
        }

        result
    }
}

fn main() {}
