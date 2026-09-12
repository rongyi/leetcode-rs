struct Solution;

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let n = tops.len();
        let candidates = [tops[0], bottoms[0]];
        let mut ans = i32::MAX;

        for &target in &candidates {
            let mut top_rot = 0;
            let mut bot_rot = 0;
            let mut possible = true;

            for i in 0..n {
                if tops[i] != target && bottoms[i] != target {
                    possible = false;
                    break;
                } else if tops[i] != target {
                    // tops[i] 需要旋转才能变成 target
                    top_rot += 1;
                } else if bottoms[i] != target {
                    // bottoms[i] 需要旋转才能变成 target
                    bot_rot += 1;
                }
                // 如果两面都等于 target，不需要旋转
            }

            if possible {
                ans = ans.min(top_rot.min(bot_rot));
            }
        }

        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}

fn main() {}
