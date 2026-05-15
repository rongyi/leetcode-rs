struct Solution;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut out = vec![];
        let mut cur = vec![];
        let mut visited = vec![false; nums.len()];

        Self::do_permute(&nums, &mut cur, &mut out, &mut visited, 0);

        out
    }

    fn do_permute(
        nums: &Vec<i32>,
        cur: &mut Vec<i32>,
        out: &mut Vec<Vec<i32>>,
        visited: &mut Vec<bool>,
        pos: usize,
    ) {
        if visited.iter().all(|v| *v) {
            out.push(cur.clone());
            return;
        }
        for i in 0..nums.len() {
            if !visited[i] {
                // the dedup trick, ordered
                if i > 0 && nums[i] == nums[i - 1] && !visited[i - 1] {
                    continue;
                }
                visited[i] = true;
                cur.push(nums[i]);

                Self::do_permute(nums, cur, out, visited, i);

                cur.pop();
                visited[i] = false;
            }
        }
    }
}

fn main() {}
