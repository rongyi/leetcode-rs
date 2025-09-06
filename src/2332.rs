struct Solution;

impl Solution {
    pub fn latest_time_catch_the_bus(
        mut buses: Vec<i32>,
        mut passengers: Vec<i32>,
        capacity: i32,
    ) -> i32 {
        buses.sort_unstable();
        passengers.sort_unstable();
        let m = buses.len();
        let n = passengers.len();
        let mut j = 0;
        for i in 0..m {
            let mut acc = 0;
            while j < n && passengers[j] <= buses[i] && acc < capacity {
                acc += 1;
                j += 1;
            }
            if i == m - 1 {
                if acc < capacity {
                    let mut t = buses[i];

                    for k in (0..j).rev() {
                        if passengers[k] == t {
                            t -= 1;
                        } else {
                            break;
                        }
                    }

                    return t;
                } else {
                    // full be the front of last chunk
                    let mut t = passengers[j - 1] - 1;
                    for k in (0..j - 1).rev() {
                        if passengers[k] == t {
                            t -= 1;
                        } else {
                            break;
                        }
                    }
                    return t;
                }
            }
        }
        -1
    }
}

fn main() {}
