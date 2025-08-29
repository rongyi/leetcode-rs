struct Solution;

impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut stk: Vec<(i32, i32)> = Vec::new();
        stk.push((nums[sz - 1], 0));
        let mut ret = 0;
        for i in (0..sz - 1).rev() {
            let mut cnt = 0;

            // Explanation
            // dp[i] means the number of element A[i] can eat on its right.
            // More precisely, the number of rounds for an element A[i],
            // to completely eat whatever it can eat on the right of A[i],
            // if it is possible.

            // Iterative input array A reversely,
            // If A[i] is bigger the last element A[j] of stack,
            // this means A[i] can eat that element,
            // Then update dp[i] to be max of dp[i] + 1 and dp[j].
            while !stk.is_empty() && stk.last().unwrap().0 < nums[i] {
                cnt += 1;
                cnt = cnt.max(stk.last().unwrap().1);
                stk.pop();
                println!("here: {}, cnt: {}", i, cnt);
            }

            ret = ret.max(cnt);
            stk.push((nums[i], cnt));
        }

        ret
    }
}

fn main() {
    let input = vec![8, 7, 6, 5, 4, 3, 2, 1];
    let v = Solution::total_steps(input);
    println!("answer: {}", v);
}
