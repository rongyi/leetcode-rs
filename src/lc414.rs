struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut top3: Vec<Option<i32>> = vec![None, None, None];
        for num in nums {
            if let Some(val) = top3[0] {
                if num == val {
                    continue;
                }
            }
            if let Some(val) = top3[1] {
                if num == val {
                    continue;
                }
            }
            if let Some(val) = top3[2] {
                if num == val {
                    continue;
                }
            }
            if top3[0].is_none() || num > top3[0].unwrap() {
                top3[2] = top3[1];
                top3[1] = top3[0];
                top3[0] = Some(num);
            } else if top3[1].is_none() || num > top3[1].unwrap() {
                top3[2] = top3[1];
                top3[1] = Some(num);
            } else if top3[2].is_none() || num > top3[2].unwrap() {
                top3[2] = Some(num);
            }
        }

        top3[2].unwrap_or_else(|| top3[0].unwrap())
    }
}

fn main() {}
