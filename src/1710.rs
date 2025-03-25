#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut box_types = box_types;
        // Sort by number of units per box in descending order
        box_types.sort_by(|a, b| b[1].cmp(&a[1]));

        let mut remaining_size = truck_size;
        let mut total_units = 0;

        for box_type in box_types {
            let number_of_boxes = box_type[0];
            let units_per_box = box_type[1];

            // Take as many boxes as possible
            let boxes_to_take = remaining_size.min(number_of_boxes);
            total_units += boxes_to_take * units_per_box;
            remaining_size -= boxes_to_take;

            if remaining_size == 0 {
                break;
            }
        }

        total_units
    }
}

fn main() {}
