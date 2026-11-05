struct Solution;

impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        // https://leetcode.com/problems/numbers-at-most-n-given-digit-set/discuss/168439/C%2B%2B-O(logN)-Clear-code-with-explanation
        // Take N = 2563, D = {"1", "2", "6"} as an example,
        // The first loop handles the count of x, xx, xxx which x belongs to D. the
        // sum is 3^1 + 3^2 + 3^3. The second loop handles xxxx from left most
        // digit. For example, count of 1xxx is 3^3 count of 21xx is 3^2 count of
        // 22xx is 3^2

        // If the elements of D can compose entired N, answer + 1
        // Although it could be coded in a single loop, the logic would be unclear
        // to me. I keep them seperated.
        let s: Vec<char> = n.to_string().chars().collect();
        let digits: Vec<char> = digits
            .into_iter()
            .map(|s| s.chars().next().unwrap())
            .collect();
        let ssz = s.len();
        let n = digits.len();
        let mut ret = 0;

        for i in 1..ssz {
            ret += n.pow(i as u32);
        }
        for i in 0..ssz {
            let mut has_same_num = false;
            for &c in digits.iter() {
                if c < s[i] {
                    ret += n.pow((ssz - i - 1) as u32);
                } else if c == s[i] {
                    has_same_num = true;
                }
            }

            // this means we can continue if we find the digit under current postion in candidate list
            // from highest to lowest
            // for the 2563 case, we first find there is 2, so we fix this postion, and continue
            // the lower pos, if not, we can not continue, because this is the only case
            if !has_same_num {
                return ret as i32;
            }
        }

        ret as i32 + 1
    }
}

fn main() {}
