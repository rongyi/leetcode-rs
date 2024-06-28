#![allow(dead_code)]

struct Solution;

impl Solution {
    // https://leetcode.com/problems/smallest-rotation-with-highest-score/solutions/118839/simple-java-o-n-solution/
    // This is really elegant!
    // Add a few comments to the solution:

    // For every number A[i] at the original index i, we calculate two K numbers:

    //     when we get one score. This will all follow the same rule: when the original ith number is rotated from index 0 to index n-1. The number of rotations required is (i + 1) % n.
    //     when we lose one score. This will happen when A[i] is rotated from A[i]th to (A[i]-1)th position. The number of rotations required is (i + 1 - A[i] + n) % n.

    // For any other rotation number K which is not the above 2 numbers, the score won't change for the given number A[i].

    // The special case is the number 0. The rotation won't change its score. The good part is, the above two required rotation numbers will be same when A[i] is 0, making it increase and decrease at the same rotation number, which results in a no-op.

    // The array a is used to accumulate the score changes. The index is the rotation count. We need to find the max prefix sum and return the index.
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut change = vec![0; sz];
        // treat it like interval
        for (i, &num) in nums.iter().enumerate() {
            println!("process index: {}, value: {}", i, num);
            let start = (i + 1) % sz;
            let end = (i + 1 - num as usize + sz) % sz;
            println!("start: {}, end: {}", start, end);
            change[start] += 1;
            change[end] -= 1;
        }
        let mut cnt = 0;
        let mut max_cnt = -1;
        let mut ret = 0;

        for (i, &num) in change.iter().enumerate() {
            cnt += num;
            if cnt > max_cnt {
                max_cnt = cnt;
                ret = i;
            }
        }

        ret as i32
    }
}

fn main() {}
