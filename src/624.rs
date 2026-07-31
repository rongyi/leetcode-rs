struct Solution;

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut max_dist = 0;

        // Initialize global min and max with the first array's boundaries
        let mut min_val = arrays[0][0];
        let mut max_val = arrays[0][arrays[0].len() - 1];

        // Process starting from the second array
        for arr in arrays.iter().skip(1) {
            let current_min = arr[0];
            let current_max = arr[arr.len() - 1];

            // Calculate potential maximum distance with elements from different arrays
            max_dist = max_dist.max((current_max - min_val).abs());
            max_dist = max_dist.max((max_val - current_min).abs());

            // Update global min and max
            min_val = min_val.min(current_min);
            max_val = max_val.max(current_max);
        }

        max_dist
    }
}

fn main() {}
