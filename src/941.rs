struct Solution;

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let sz = arr.len();
        if sz < 3 {
            return false;
        }
        if arr[0] >= arr[1] {
            return false;
        }
        let mut has_right = false;
        for i in 1..sz {
            if arr[i] == arr[i - 1] {
                return false;
            } else if arr[i] > arr[i - 1] {
                if has_right {
                    return false;
                }
            } else {
                has_right = true;
            }
        }

        has_right
    }
}

fn main() {}
