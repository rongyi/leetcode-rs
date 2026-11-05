
struct Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut out = vec![];
        let mut cur = vec![];

        Self::backtrack(&nums, 0, &mut cur, &mut out);

        out
    }
    fn backtrack(nums: &Vec<i32>, start: usize, cur: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
        out.push(cur.clone());

        for i in start..nums.len() {
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }
            cur.push(nums[i]);

            Self::backtrack(nums, i + 1, cur, out);

            cur.pop();
        }
    }
}

fn main() {}
