#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut prev_finish = 0;
        let mut total_waiting_time = 0.0;

        for customer in customers.iter() {
            let arrival_time = customer[0];
            let cooking_time = customer[1];

            // Update current time to at least the arrival time
            // if chef is idle, finish before arrival time, make finish just as arrival time
            prev_finish = prev_finish.max(arrival_time);

            // Calculate waiting time for this customer
            let finish_time = prev_finish + cooking_time;
            let waiting_time = finish_time - arrival_time;

            total_waiting_time += waiting_time as f64;

            // Update current time to when this customer's order is complete
            prev_finish = finish_time;
        }

        total_waiting_time / customers.len() as f64
    }
}
fn main() {}
