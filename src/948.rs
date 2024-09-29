struct Solution;

impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        tokens.sort_unstable();
        let mut score = 0;
        let mut max_score = 0;
        let mut left = 0;
        let mut right = tokens.len() as i32 - 1;

        while left <= right && (power >= tokens[left as usize] || score > 0) {
            while left <= right && power >= tokens[left as usize] {
                power -= tokens[left as usize];
                score += 1;
                left += 1;
            }
            max_score = max_score.max(score);

            if left <= right && score > 0 {
                power += tokens[right as usize];
                score -= 1;
                right -= 1;
            }
        }

        max_score
    }
}

fn main() {}
