struct Solution;

impl Solution {
    /// LeetCode 556: Next Greater Element III
    ///
    /// Find the smallest integer > n with the same digits.
    /// Uses the standard next-permutation algorithm: find pivot,
    /// swap with the next larger digit to its right, reverse suffix.
    pub fn next_greater_element(n: i32) -> i32 {
        let mut digits: Vec<u8> = n.to_string().bytes().map(|b| b - b'0').collect();
        let sz = digits.len();

        // Step 1: rightmost digit smaller than the one after it.
        let mut i = sz as isize - 2;
        while i >= 0 && digits[i as usize] >= digits[i as usize + 1] {
            i -= 1;
        }
        if i < 0 {
            return -1; // descending order → no larger permutation
        }
        println!("i: {i}");

        // Step 2: smallest digit to the right that is > digits[i].
        let mut j = sz - 1;
        while digits[j] <= digits[i as usize] {
            j -= 1;
        }
        println!("j: {j}");

        // Step 3: swap, then reverse the suffix.
        digits.swap(i as usize, j);
        // println!("after swap: {:?}", digits);
        digits[i as usize + 1..].reverse();
        // println!("after reverse: {:?}", digits);

        // Convert back, check i32 overflow.
        let val: i64 = digits.iter().fold(0i64, |acc, &d| acc * 10 + d as i64);
        if val > i32::MAX as i64 {
            -1
        } else {
            val as i32
        }
    }
}

fn main() {
    let v = Solution::next_greater_element(43521);
    println!("{v}");
}
