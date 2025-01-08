#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_candies(
        mut status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut ret = 0;
        let mut has_change = true;
        let mut initial_boxes: Vec<usize> = initial_boxes.into_iter().map(|x| x as usize).collect();

        while !initial_boxes.is_empty() && has_change {
            has_change = false;
            let mut new_box: Vec<usize> = Vec::new();
            for &b in initial_boxes.iter() {
                if status[b] == 1 {
                    has_change = true;
                    // using key to unlock other box

                    for &k in keys[b].iter() {
                        status[k as usize] = 1;
                    }
                    for &id in contained_boxes[b].iter() {
                        new_box.push(id as usize);
                    }

                    ret += candies[b];
                } else {
                    new_box.push(b);
                }
            }

            initial_boxes = new_box;
        }

        ret
    }
}


fn main() {}
