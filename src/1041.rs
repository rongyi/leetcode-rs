struct Solution;

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        let mut cur_dir = 0; // 0: north, 1: east, 2: south, 3: west

        // Directions array: [dx, dy] for [north, east, south, west]
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for c in instructions.chars() {
            match c {
                'G' => {
                    x += dirs[cur_dir].0;
                    y += dirs[cur_dir].1;
                }
                'L' => cur_dir = (cur_dir + 3) % 4,
                'R' => cur_dir = (cur_dir + 1) % 4,
                _ => unreachable!(),
            }
        }
        (x == 0 && y == 0) || cur_dir != 0
    }
}

fn main() {}
