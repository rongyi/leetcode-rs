#![allow(dead_code)]

struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut deque: VecDeque<(i32, i32)> = VecDeque::new(); // Store pairs of (y-x, x)
        let mut max_val = i32::MIN;

        // Let me clarify what "y - x" means in the context of this problem.

        // ## Understanding the Equation

        // For any two points (xi, yi) and (xj, yj) where i < j (thus xi < xj), the value we want to maximize is:
        // yi + yj + |xi - xj|

        // Since xi < xj, this becomes:
        // yi + yj + (xj - xi)

        // We can rearrange this as:
        // (yi - xi) + (yj + xj)

        // For any given point j, the term (yj + xj) is fixed. So to maximize the whole expression, we need to find a previous point i such that:
        // 1. xj - xi ≤ k (distance constraint)
        // 2. (yi - xi) is as large as possible

        // ## What "y - x" Represents

        // In our algorithm:
        // - "y - x" represents the value (yi - xi) for a point i
        // - For each current point j, we look for the maximum (yi - xi) value among all previous valid points
        // - This maximum (yi - xi) value, when added to (yj + xj), gives us the maximum possible value of our equation

        // So "y - x" is a key component that helps us efficiently find the maximum value of the equation without having to check all pairs of points.

        // In summary, by storing and tracking (y - x) values in our deque, we can quickly determine which previous point will give us the maximum equation value when paired with the current point.

        for point in points {
            let curr_x = point[0];
            let curr_y = point[1];

            // Remove points that are too far away
            while !deque.is_empty() && curr_x - deque.front().unwrap().1 > k {
                deque.pop_front();
            }

            // Check if we can update the max value
            if !deque.is_empty() {
                let (y_minus_x, _) = deque.front().unwrap();
                max_val = max_val.max(curr_y + curr_x + y_minus_x);
            }

            // Remove points that are not optimal (points with smaller y-x values)
            while !deque.is_empty() && deque.back().unwrap().0 <= curr_y - curr_x {
                deque.pop_back();
            }

            // Add current point
            deque.push_back((curr_y - curr_x, curr_x));
        }

        max_val
    }
}
fn main() {}
