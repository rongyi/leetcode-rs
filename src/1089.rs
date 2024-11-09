struct Solution;

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let sz = arr.len();
        let mut i = 0;
        while i < sz {
            if arr[i] == 0 {
                // ok, shit all num alfter
                for j in (i + 1..sz).rev() {
                    arr[j] = arr[j - 1];
                }
                if i + 1 < sz {
                    arr[i + 1] = 0;
                }
                i += 2;
            } else {
                i += 1;
            }
        }
    }
}

fn main() {}
