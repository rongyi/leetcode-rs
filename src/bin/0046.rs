struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut out = vec![];
        let mut visited = vec![false; nums.len()];

        let mut cur = vec![];

        Self::do_permute(&nums, &mut out, &mut visited, &mut cur);

        out
    }
    fn do_permute(
        nums: &Vec<i32>,
        out: &mut Vec<Vec<i32>>,
        visited: &mut Vec<bool>,
        cur: &mut Vec<i32>,
    ) {
        if visited.iter().all(|t| *t) {
            out.push(cur.clone());
            return;
        }
        for i in 0..nums.len() {
            if !visited[i] {
                visited[i] = true;
                cur.push(nums[i]);
                Self::do_permute(nums, out, visited, cur);

                cur.pop();
                visited[i] = false;
            }
        }
    }
}

fn main() {}
