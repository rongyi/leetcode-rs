struct Solution;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let s: Vec<char> = s.chars().collect();
        let sz = s.len();
        let mut ret: Vec<i32> = vec![i32::MAX; sz];

        let mut distance = i32::MAX;
        for i in 0..sz {
            if s[i] == c {
                ret[i] = 0;
                distance = 0;
            } else {
                if distance != i32::MAX {
                    distance += 1;
                    ret[i] = distance;
                }
            }
        }
        distance = i32::MAX;
        for i in (0..sz).rev() {
            if s[i] == c {
                ret[i] = 0;
                distance = 0;
            } else {
                if distance != i32::MAX {
                    distance += 1;
                    ret[i] = ret[i].min(distance);
                }
            }
        }

        ret
    }
}

fn main() {}
