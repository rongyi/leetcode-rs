struct Solution;

impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let cars = cars as i64;
        let mut l = 0;
        let mut r = cars * cars * 100;
        let mut ret = r;
        while l <= r {
            let mid = l + (r - l) / 2;
            if Self::check(mid, &ranks, cars) {
                ret = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        ret
    }
    fn check(cur: i64, ranks: &Vec<i32>, mut cars: i64) -> bool {
        for &r in ranks.iter() {
            if cars <= 0 {
                break;
            }
            let repair = ((cur / r as i64) as f64).sqrt().floor() as i64;
            cars -= repair;
        }

        cars <= 0
    }
}

fn main() {}
