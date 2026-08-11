struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();

        for &ast in asteroids.iter() {
            let current = ast;
            let mut exploded = false;

            // While there's a collision possibility: stack top moves right (positive)
            // and current moves left (negative)
            while !stack.is_empty() && stack.last().unwrap() > &0 && current < 0 {
                let top = stack.pop().unwrap();

                // Compare sizes
                if top > -current {
                    // Top is larger, current explodes
                    stack.push(top);
                    exploded = true;
                    break;
                } else if top == -current {
                    // Both explode
                    exploded = true;
                    break;
                }
                // current is larger, top explodes, continue checking with new top
            }

            // If current survived the collisions, push it
            if !exploded {
                stack.push(current);
            }
        }

        stack
    }
}

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut to_left = Vec::new();
        let mut to_right = Vec::new();

        for &star in asteroids.iter() {
            if star > 0 {
                to_right.push(star);
            } else {
                let mut left_exist = true;
                while !to_right.is_empty() {
                    let top = (*to_right.last().unwrap()).abs();
                    if top < star.abs() {
                        to_right.pop();
                    } else if top == star.abs() {
                        to_right.pop();
                        left_exist = false;
                        break;
                    } else {
                        // the smaller one to left is exploded, do nothing
                        left_exist = false;
                        break;
                    }
                }
                if left_exist {
                    to_left.push(star);
                }
            }
        }
        to_left.extend(to_right.iter());

        to_left
    }
}

fn main() {}
