#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        use std::cmp::Ordering;
        use std::collections::BinaryHeap;

        #[derive(PartialEq)]
        struct Class {
            pass: i32,
            total: i32,
        }

        impl Class {
            fn new(pass: i32, total: i32) -> Self {
                Class { pass, total }
            }

            fn profit(&self) -> f64 {
                let curr_ratio = self.pass as f64 / self.total as f64;
                let new_ratio = (self.pass + 1) as f64 / (self.total + 1) as f64;
                new_ratio - curr_ratio
            }
        }

        impl Eq for Class {}

        impl PartialOrd for Class {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.profit().partial_cmp(&other.profit())
            }
        }

        impl Ord for Class {
            fn cmp(&self, other: &Self) -> Ordering {
                self.partial_cmp(other).unwrap()
            }
        }

        let mut heap = BinaryHeap::new();

        // Add all classes to the heap
        for class in classes.iter() {
            let (pass, total) = (class[0], class[1]);
            heap.push(Class::new(pass, total));
        }

        // Assign extra students to classes with highest profit
        for _ in 0..extra_students {
            if let Some(mut class) = heap.pop() {
                class.pass += 1;
                class.total += 1;
                heap.push(class);
            }
        }

        // Calculate the average ratio
        let mut sum = 0.0;
        let n = heap.len() as f64;

        while let Some(class) = heap.pop() {
            sum += class.pass as f64 / class.total as f64;
        }

        sum / n
    }
}

fn main() {}
