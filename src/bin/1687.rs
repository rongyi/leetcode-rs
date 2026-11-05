#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn box_delivering(
        boxes: Vec<Vec<i32>>,
        ports_count: i32,
        max_boxes: i32,
        max_weight: i32,
    ) -> i32 {
        let max_boxes = max_boxes as usize;
        let sz = boxes.len();
        let mut dp = vec![0; sz + 1];

        // slidingwindow的左侧
        let mut i = 0;
        let mut cur_cost = 1;
        let mut cur_weight = 0;

        // sliding window的右侧，闭区间
        for j in 0..sz {
            // 港口目的地增加了
            if j == 0 || boxes[j][0] != boxes[j - 1][0] {
                cur_cost += 1;
            }
            cur_weight += boxes[j][1];
            // 最后一个判断是如果dp[i + 1] == dp[i] 那么到 i 和到 i -
            // 1这里跑的趟数是一样的，那就放一起跑了吧，
            // 因为放在后面的话，可能会多一趟，目的一样还好说，目的不一样就要多跑一趟了
            while j - i + 1 > max_boxes as usize
                || cur_weight > max_weight
                || (i < j && dp[i + 1] == dp[i])
            {
                cur_weight -= boxes[i][1];

                // 我们只在港口目的地不一样的时候增加了，相应的也去减少
                if boxes[i][0] != boxes[i + 1][0] {
                    cur_cost -= 1;
                }
                i += 1;
            }
            dp[j + 1] = dp[i] + cur_cost;
        }
        dp[sz]
    }
}
fn main() {}
