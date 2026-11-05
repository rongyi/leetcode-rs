struct Solution;

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let sz = seats.len();
        let mut to_right = vec![i32::MAX; sz];
        let mut to_left = vec![i32::MAX; sz];
        let mut prev = -1;
        for i in 0..sz {
            if seats[i] == 1 {
                prev = i as i32;
            } else {
                if prev != -1 {
                    to_right[i] = i as i32 - prev;
                }
            }
        }

        prev = -1;
        for i in (0..sz).rev() {
            if seats[i] == 1 {
                prev = i as i32;
            } else {
                if prev != -1 {
                    to_left[i] = prev - i as i32;
                }
            }
        }

        to_right
            .into_iter()
            .zip(to_left.into_iter())
            .enumerate()
            .filter(|&(idx, (_a, _b))| seats[idx] == 0)
            .map(|(_idx, (a, b))| a.min(b))
            .max()
            .unwrap()
    }
}

fn main() {}
