struct Solution;

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut preorder: Vec<char> = preorder.chars().collect();
        preorder.push(',');
        let mut capacity = 1;
        for (i, c) in preorder.iter().enumerate() {
            if *c != ',' {
                continue;
            }
            capacity -= 1;
            if capacity < 0 {
                return false;
            }

            if preorder[i - 1] != '#' {
                capacity += 2;
            }
        }

        capacity == 0
    }
}

fn main() {}
