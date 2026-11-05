struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut cur = vec![];
        let mut out = vec![];

        Self::backtrack(0, &nums, &mut cur, &mut out);

        out
    }

    fn backtrack(start: usize, nums: &Vec<i32>, cur: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
        out.push(cur.clone());

        for i in start..nums.len() {
            cur.push(nums[i]);

            Self::backtrack(i + 1, nums, cur, out);

            cur.pop();
        }
    }
}

fn main() {}
