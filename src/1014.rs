struct Solution;

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut best = values[0] + 0; // values[i] + i，i 从 0 开始
        let mut ans = i32::MIN;

        // 公式变形
        // score = values[i] + values[j] + i - j
        //       = (values[i] + i) + (values[j] - j)
        for j in 1..values.len() {
            // 用当前 j 和之前最优的 i 组合
            ans = ans.max(best + values[j] - j as i32);
            // 更新 best，把当前位置 j 也纳入候选 i
            best = best.max(values[j] + j as i32);
        }

        ans
    }
}

fn main() {}
