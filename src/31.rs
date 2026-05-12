struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut pivot: Option<usize> = None;
        let sz = nums.len();
        for i in (0..sz - 1).rev() {
            if nums[i] < nums[i + 1] {
                pivot = Some(i);
                break;
            }
        }
        if let Some(pivot) = pivot {
            for j in (0..sz).rev() {
                // found
                if nums[j] > nums[pivot] {
                    nums.swap(pivot, j);
                    break;
                }
            }
            nums[pivot + 1..].reverse();
        } else {
            nums.reverse();
        }
    }
}

fn main() {
    let mut input = vec![1, 2, 3];
    Solution::next_permutation(&mut input);
}
