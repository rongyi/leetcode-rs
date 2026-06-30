pub struct Solution;

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut start = 1;
        let mut end = n;
        while start <= end {
            let mid = start + (end - start) / 2;
            let check = guess(mid);
            if check == 0 {
                return mid;
            } else if check < 0 {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        -1
    }
}

fn main() {}
