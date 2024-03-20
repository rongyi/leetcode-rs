struct Solution;

impl Solution {
    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        let n = machines.len();
        let mut prefix_sum = vec![0; n + 1];
        for (i, &dress) in machines.iter().enumerate() {
            prefix_sum[i + 1] = prefix_sum[i] + machines[i];
        }
        if prefix_sum[n] % n as i32 != 0 {
            return -1;
        }

        let avg = prefix_sum[n] / n as i32;
        let mut ret = 0;
        for i in 0..n {
            // Think about the machine i, after we make all machines have the same
            // dresses, how many dresses will be passed through machine i?
            // Let's denote the current sum of dresses of machines [0...i-1] as
            // leftSums[i], and the current sum of dresses of machines [i+1...n-1] as
            // rightSums[i].

            // Let's denote the expected sum of dresses of machines [0...i-1] as
            // expLeft, which means after all dresses are equally distributed, the sum
            // of address in machines [0...i-1] should be expLeft. The same logic
            // applies to machines [i+1...n-1], denoted as expRight.

            // Then the above question should be clearly answered. If expLeft is
            // larger than leftSums[i], that means no matter how you move the dresses,
            // there will be at least expLeft - leftSums[i] dresses being moved to
            // left of machine i, which means pass through machine i. For the right
            // machines of machine i, the logic remains the same. So we could conclude
            // that the minimum dresses passed through machine i will be:

            // left = expLeft > leftSums[i] ? expLeft - leftSums[i] : 0;
            // right = expRight > rightSums[i] ? expRight - rightSums[i] : 0;
            // total = left + right;

            let expect_left = i as i32 * avg;
            let expect_right = (n - i - 1) as i32 * avg;
            let mut left = 0;
            let mut right = 0;
            // i + 1 -> n -1
            // prefix_sum[n - 1 + 1] - prefix[i + 1 - 1 + 1]
            let right_sum = prefix_sum[n] - prefix_sum[i + 1];
            if expect_left > prefix_sum[i] {
                left = expect_left - prefix_sum[i];
            }
            if expect_right > right_sum {
                right = expect_right - right_sum;
            }
            ret = ret.max(left + right);
        }

        ret
    }
}

fn main() {}
