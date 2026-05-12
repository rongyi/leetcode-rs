struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = nums[mid as usize];

            if target == mid_val {
                return mid;
            }
            if target < mid_val {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        // when not found, return left
        left
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_insert() {
        let input = vec![1, 3, 5, 6];
        let target = 2;
        let idx = super::Solution::search_insert(input, target);

        assert_eq!(idx, 1);
    }
}

fn main() {}
