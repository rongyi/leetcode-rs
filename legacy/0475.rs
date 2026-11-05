struct Solution;

impl Solution {
    /*
    Example:    h = house,  * = heater  M = INT_MAX

            h   h   h   h   h   h   h   h   h    houses
            1   2   3   4   5   6   7   8   9    index
            *           *       *                heaters

            0   2   1   0   1   0   -   -   -    (distance to nearest RHS heater)
            0   1   2   0   1   0   1   2   3    (distance to nearest LHS heater)

            0   1   1   0   1   0   1   2   3    (res = minimum of above two)

    Result is maximum value in res, which is 3.
    */
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

        // scan left side
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

fn main() {
    let mut a = vec![1, 1, 1, 1, 2];
    let b = a.iter().filter(|&&i| i == 1).count();
    println!("{:?}", b);
}
