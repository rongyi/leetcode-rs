struct Solution;

impl Solution {
    pub fn minimum_operations_to_make_equal(x: i32, y: i32) -> i32 {
        if x == y {
            return 0;
        }
        let mut queue = std::collections::VecDeque::new();
        let mut visited = std::collections::HashSet::new();
        queue.push_back((x, 0));
        visited.insert(x);

        while let Some((current, steps)) = queue.pop_front() {
            if current == y {
                return steps;
            }

            let next_steps = steps + 1;
            let candidates = [
                current + 1,
                current - 1,
                if current % 11 == 0 {
                    current / 11
                } else {
                    i32::MAX
                },
                if current % 5 == 0 {
                    current / 5
                } else {
                    i32::MAX
                },
            ];

            for &next in &candidates {
                if next != i32::MAX && !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back((next, next_steps));
                }
            }
        }

        -1
    }
}

fn main() {}
