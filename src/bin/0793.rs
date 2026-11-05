#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        // This solution takes advantage of the last observation mentioned above,
        // where we use binary search to find the largest integers x and y such
        // that numOfTrailingZeros(x) <= K and numOfTrailingZeros(y) <= K-1, respectively.
        // Then the factorial of all integers in the range (y, x] (left exclusive, right inclusive)
        // will have K trailing zeros. Therefore the total number of non-negative integers whose
        // factorials have K trailing zeros will be given by x - y
        (Self::binary_search(k) - Self::binary_search(k - 1)) as i32
    }
    pub fn binary_search(k: i32) -> i64 {
        let k = k as i64;
        let mut left = 0i64;
        let mut right = 5 * (k + 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            let cnt = Self::tzero(mid);
            if cnt <= k {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        right
    }
    fn tzero(mut x: i64) -> i64 {
        let mut ret = 0;
        while x > 0 {
            ret += x / 5;
            x /= 5;
        }

        ret
    }
}

fn main() {}
