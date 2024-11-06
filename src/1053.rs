struct Solution;

impl Solution {
    pub fn prev_perm_opt1(mut arr: Vec<i32>) -> Vec<i32> {
        let sz = arr.len();
        let mut i = (sz - 2) as i32;
        while i >= 0 && arr[i as usize] <= arr[(i + 1) as usize] {
            i -= 1;
        }
        if i < 0 {
            return arr;
        }
        let mut j = sz - 1;
        while arr[j] >= arr[i as usize] {
            j -= 1;
        }
        while j > 0 && arr[j - 1] == arr[j] {
            j -= 1;
        }
        arr.swap(i as usize, j);

        arr
    }
}

fn main() {}
