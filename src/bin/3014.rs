struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let n = word.len() as i32;
        let mut total_pushes = 0;

        // We distribute the n distinct characters across 8 keys
        // Characters 1-8: 1 push each
        // Characters 9-16: 2 pushes each
        // Characters 17-24: 3 pushes each
        // Characters 25-26: 4 pushes each

        for i in 0..n {
            // (i / 8) gives the "round" or the number of extra pushes needed
            // Adding 1 because the first round (0) costs 1 push.
            total_pushes += (i / 8) + 1;
        }

        total_pushes
    }
}

fn main() {}
