struct Solution;

impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        if derived.len() == 1 {
            return derived[0] == 0;
        }
        if Self::chain_action(&derived, 1, 0, 0 ^ derived[0])
            || Self::chain_action(&derived, 1, 1, 1 ^ derived[0])
        {
            return true;
        }
        false
    }
    fn chain_action(derived: &Vec<i32>, pos: usize, origin0: i32, next_val: i32) -> bool {
        if pos == derived.len() - 1 {
            return *derived.last().unwrap() == next_val ^ origin0;
        }
        Self::chain_action(derived, pos + 1, origin0, next_val ^ derived[pos])
    }
}

fn main() {}
