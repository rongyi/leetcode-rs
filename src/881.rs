struct Solution;

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let sz = people.len();
        let mut ret = 0;
        let mut i = 0;
        let mut j = sz as i32 - 1;
        // if only one people left, we still need to add one boat
        // so the equal case should be included
        while i <= j {
            if people[i as usize] + people[j as usize] <= limit {
                i += 1;
            }

            j -= 1;
            ret += 1;
        }

        ret
    }
}

fn main() {}
