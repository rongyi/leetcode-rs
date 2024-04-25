#![allow(dead_code)]

struct Solution;

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

fn main() {
    let input = vec![5, 10, -5];
    let val = Solution::asteroid_collision(input);
    println!("{:?}", val);
}
