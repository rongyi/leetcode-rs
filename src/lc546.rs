struct Solution;

impl Solution {
    // so dame hard, more detail can be found:
    // https://leetcode.com/problems/remove-boxes/solutions/1402561/c-java-python-top-down-dp-clear-explanation-with-picture-clean-concise/
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let m = boxes.len();
        let mut dp = vec![vec![vec![0; m]; m]; m];

        Self::recur(&boxes, &mut dp, 0, m as i32 - 1, 0)
    }
    fn recur(boxes: &Vec<i32>, dp: &mut Vec<Vec<Vec<i32>>>, mut l: i32, r: i32, mut k: i32) -> i32 {
        if l > r {
            return 0;
        }
        let ul = l as usize;
        let ur = r as usize;
        let uk = k as usize;
        if dp[ul][ur][uk] > 0 {
            return dp[ul][ur][uk];
        }
        let l_origin = l;
        let k_origin = k;
        while l + 1 <= r && boxes[l as usize] == boxes[(l + 1) as usize] {
            l += 1;
            k += 1;
        }
        let mut ret = (k + 1) * (k + 1) + Self::recur(boxes, dp, l + 1, r, 0);
        for m in l + 1..=r {
            if boxes[m as usize] == boxes[l as usize] {
                ret = ret.max(
                    Self::recur(boxes, dp, m, r, k + 1) + Self::recur(boxes, dp, l + 1, m - 1, 0),
                );
            }
        }
        dp[l_origin as usize][r as usize][k_origin as usize] = ret;
        ret
    }
}

fn main() {}
