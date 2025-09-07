struct Solution;

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        // first filter
        let origin_lr: Vec<char> = start.chars().filter(|c| *c != '_').collect();
        let target_lr: Vec<char> = target.chars().filter(|c| *c != '_').collect();
        let origin_space_cnt: usize = start.chars().filter(|c| *c == '_').count();
        let target_space_cnt: usize = target.chars().filter(|c| *c == '_').count();

        if origin_lr != target_lr || origin_space_cnt != target_space_cnt {
            return false;
        }
        let origin: Vec<char> = start.chars().collect();
        let target: Vec<char> = target.chars().collect();
        let sz = origin.len();
        let mut i = 0;
        let mut j = 0;
        while i < sz && j < sz {
            while i < sz && origin[i] == '_' {
                i += 1;
            }
            while j < sz && target[j] == '_' {
                j += 1;
            }
            println!("compare index: {i}, {j}");

            if i == sz && j == sz {
                return true;
            }
            // must be same
            if i < sz && j < sz && origin[i] != target[j] {
                return false;
            }

            // L can only be shift to left, so original index must be great than or equal to target
            if i < sz && origin[i] == 'L' && i < j {
                return false;
            }
            if i < sz && origin[i] == 'R' && i > j {
                return false;
            }

            // valid case
            i += 1;
            j += 1;
        }

        true
    }
}

fn main() {
    Solution::can_change("_LL__R__R_".to_string(), "L___L___RR".to_string());
}
