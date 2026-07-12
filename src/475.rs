struct Solution;

impl Solution {
    pub fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        houses.sort();
        heaters.sort();
        let m = houses.len();
        let n = heaters.len();
        let mut distance = vec![i32::MAX; m];
        // find heater at right side
        let mut i = 0;
        let mut j = 0;
        while i < m && j < n {
            if houses[i] <= heaters[j] {
                distance[i] = heaters[j] - houses[i];
                i += 1;
            } else {
                j += 1;
            }
        }

        // and left side
        let mut i: i32 = (m - 1) as i32;
        let mut j: i32 = (n - 1) as i32;
        while i >= 0 && j >= 0 {
            if houses[i as usize] >= heaters[j as usize] {
                distance[i as usize] =
                    distance[i as usize].min(houses[i as usize] - heaters[j as usize]);
                i -= 1;
            } else {
                j -= 1;
            }
        }

        distance.into_iter().max().unwrap()
    }
}

fn main() {}
