#![allow(dead_code)]

struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut bricks_left = bricks;
        let mut ladders_left = ladders;

        // Min-heap to store height differences where we've used ladders
        let mut ladder_uses = BinaryHeap::new();

        for i in 0..heights.len() - 1 {
            let diff = heights[i + 1] - heights[i];

            // If next building is not taller, we can proceed without resources
            if diff <= 0 {
                continue;
            }

            // Use a ladder by default for a climb
            ladder_uses.push(Reverse(diff));

            // If we've used more ladders than available
            if ladder_uses.len() > ladders_left as usize {
                // Replace the smallest ladder use with bricks
                if let Some(Reverse(smallest_ladder_use)) = ladder_uses.pop() {
                    bricks_left -= smallest_ladder_use;

                    // If we don't have enough bricks, we can't proceed further
                    if bricks_left < 0 {
                        return i as i32;
                    }
                }
            }
        }

        // We've managed to reach the last building
        return heights.len() as i32 - 1;
    }
}

fn main() {}
