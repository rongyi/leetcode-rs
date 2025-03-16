#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        let mut same_as_location = 0;
        let mut angles = Vec::new();
        let (lx, ly) = (location[0], location[1]);

        // Calculate angles for all points and count points at the same location
        for point in points {
            let (x, y) = (point[0], point[1]);
            if x == lx && y == ly {
                same_as_location += 1;
                continue;
            }

            // Calculate angle in radians and convert to degrees
            let angle_rad = (y - ly) as f64 / (x - lx) as f64;
            let mut angle_deg = angle_rad.atan() * 180.0 / std::f64::consts::PI;

            // Adjust angle based on quadrant
            if x < lx {
                angle_deg += 180.0;
            } else if y < ly {
                angle_deg += 360.0;
            }

            angles.push(angle_deg);
        }

        // Sort angles to use sliding window
        angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Duplicate angles for circular consideration
        let len = angles.len();
        for i in 0..len {
            angles.push(angles[i] + 360.0);
        }

        // Use sliding window to find maximum visible points
        let mut max_points = 0;
        let angle_f = angle as f64;
        let mut j = 0;

        for i in 0..angles.len() {
            while j < angles.len() && angles[j] - angles[i] <= angle_f {
                j += 1;
            }
            max_points = max_points.max(j - i);
        }

        // Return the result including points at the same location
        (max_points as i32) + same_as_location
    }
}

fn main() {}
