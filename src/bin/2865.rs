struct Solution;

impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let sz = max_heights.len();

        fn get_side_sums(heights: &[i32]) -> Vec<i64> {
            let sz = heights.len();
            let mut sums = vec![0i64; sz];
            let mut stack: Vec<usize> = vec![];
            let mut current_sum = 0i64;
            for i in 0..sz {
                while let Some(&top_idx) = stack.last() {
                    if heights[top_idx] > heights[i] {
                        stack.pop();
                        let prev_idx = if let Some(&prev) = stack.last() {
                            prev as i64
                        } else {
                            -1
                        };
                        current_sum -= (top_idx as i64 - prev_idx) * heights[top_idx] as i64;
                    } else {
                        break;
                    }
                }
                let prev_idx = if let Some(&prev) = stack.last() {
                    prev as i64
                } else {
                    -1
                };
                current_sum += (i as i64 - prev_idx) * heights[i] as i64;
                sums[i] = current_sum;
                stack.push(i);
            }
            sums
        }
        let left_sums = get_side_sums(&max_heights);
        let mut reverse_heights = max_heights.clone();
        reverse_heights.reverse();
        let mut right_sums = get_side_sums(&reverse_heights);
        right_sums.reverse();
        let mut ret = 0;
        for i in 0..sz {
            ret = ret.max(left_sums[i] + right_sums[i] - max_heights[i] as i64);
        }

        ret
    }
}

fn main() {}
