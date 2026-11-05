struct Solution;

impl Solution {
    pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
        let mut stones = stones;
        stones.sort_unstable();
        let n = stones.len();

        // Calculate maximum moves
        let max_moves = {
            let spaces_right = stones[n - 1] - stones[1] - (n as i32 - 2);
            let spaces_left = stones[n - 2] - stones[0] - (n as i32 - 2);
            spaces_right.max(spaces_left)
        };

        // Calculate minimum moves
        let min_moves = {
            let mut min_moves = n as i32;
            let mut j = 0;

            for i in 0..n {
                while j + 1 < n && stones[j + 1] - stones[i] < n as i32 {
                    j += 1;
                }
                let count = j - i + 1;
                let remaining = n - count;

                // Special case handling
                if count == n - 1 && stones[j] - stones[i] == n as i32 - 2 {
                    min_moves = min_moves.min(2);
                } else {
                    min_moves = min_moves.min(remaining as i32);
                }
            }
            min_moves
        };

        vec![min_moves, max_moves]
    }
}

fn main() {}
