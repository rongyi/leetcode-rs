struct Solution;

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let n = arr.len();
        if n < 3 {
            return false;
        }

        let mut i = 0;

        // Walk up: strictly increasing
        while i + 1 < n && arr[i] < arr[i + 1] {
            i += 1;
        }

        // Peak cannot be the first or last element
        if i == 0 || i == n - 1 {
            return false;
        }

        // Walk down: strictly decreasing
        while i + 1 < n && arr[i] > arr[i + 1] {
            i += 1;
        }

        // Return true if we reached the end of the array
        i == n - 1
    }
}

fn main() {}
