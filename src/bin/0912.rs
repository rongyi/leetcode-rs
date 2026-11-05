struct Solution;

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() <= 1 {
            return nums;
        }
        let mid = nums.len() / 2;
        let left = Self::sort_array(nums[..mid].to_vec());
        let right = Self::sort_array(nums[mid..].to_vec());
        Self::merge(left, right)
    }

    fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(left.len() + right.len());
        let mut left_iter = left.into_iter();
        let mut right_iter = right.into_iter();
        let mut left_peek = left_iter.next();
        let mut right_peek = right_iter.next();

        loop {
            match (left_peek, right_peek) {
                (Some(l), Some(r)) => {
                    if l <= r {
                        result.push(l);
                        left_peek = left_iter.next();
                    } else {
                        result.push(r);
                        right_peek = right_iter.next();
                    }
                }
                (Some(l), None) => {
                    result.push(l);
                    result.extend(left_iter);
                    return result;
                }
                (None, Some(r)) => {
                    result.push(r);
                    result.extend(right_iter);
                    return result;
                }
                (None, None) => return result,
            }
        }
    }
}

fn main() {}
