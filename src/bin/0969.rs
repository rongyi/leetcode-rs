struct Solution;

impl Solution {
    pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let sz = arr.len();

        // Place numbers from n down to 1 in their correct spots
        for target in (1..=sz).rev() {
            // Find current index of the target value
            let idx = arr.iter().position(|&x| x == target as i32).unwrap();

            // Skip if it's already in the correct spot
            if idx == (target - 1) {
                continue;
            }

            // Step 1: If it's not at the front (index 0), flip it to index 0
            if idx != 0 {
                result.push((idx + 1) as i32);
                arr[0..=idx].reverse();
            }

            // Step 2: Flip it to its target position at index (target - 1)
            result.push(target as i32);
            arr[0..target as usize].reverse();
        }

        result
    }
}

fn main() {}
