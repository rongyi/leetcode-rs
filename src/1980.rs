struct Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let mut result = String::new();

        for i in 0..n {
            // We use the ith character from the ith string to ensure we generate a binary string
            // that differs from all input strings in at least one position (specifically, it differs
            // from the ith string at position i). This is known as the diagonalization approach.
            //
            // The diagonalization approach works by constructing a new binary string where each bit
            // is chosen to be different from the corresponding bit in one of the input strings.
            // Specifically, for position i in our result string, we look at the ith bit of the ith
            // input string and flip it. This guarantees our result will differ from every input
            // string in at least one position (it differs from the kth string at position k).
            // This is similar to Cantor's diagonal argument, ensuring the constructed string
            // cannot be equal to any string in the input list.
            let ch = nums[i].chars().nth(i).unwrap();
            result.push(if ch == '0' { '1' } else { '0' });
        }

        result
    }
}

fn main() {}
