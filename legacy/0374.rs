struct Solution;


/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

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
                end = mid;
            } else {
                start = mid + 1;
            }
        }
        -1
    }
}

fn main() {

}
