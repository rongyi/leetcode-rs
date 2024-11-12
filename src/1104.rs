struct Solution;

impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = label;

        // Find the level of the label
        let mut level = 0;
        while (1 << level) <= label {
            level += 1;
        }

        // Build path from bottom to top
        while current > 0 {
            result.push(current);

            // Get parent level start and end
            let parent_level = level - 1;
            let level_start = 1 << (parent_level - 1);
            let level_end = (1 << parent_level) - 1;

            // If parent level is reversed, we need to find the actual parent
            if parent_level % 2 == 0 {
                let parent_position = (current / 2 - level_start);
                current = level_end - parent_position;
            } else {
                let parent_position = (level_end - current / 2);
                current = level_start + parent_position;
            }

            level -= 1;
        }

        // Reverse to get path from root to label
        result.reverse();
        result
    }
}

fn main() {}
