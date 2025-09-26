struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut cnt: BTreeMap<char, i32> = s.chars().fold(BTreeMap::new(), |mut acc, cur| {
            *acc.entry(cur).or_default() += 1;
            acc
        });
        let mut stk = Vec::new();
        let mut ret = String::new();

        for c in s.chars() {
            stk.push(c);
            *cnt.get_mut(&c).unwrap() -= 1;
            if *cnt.get(&c).unwrap() == 0 {
                cnt.remove(&c);
            }

            while !stk.is_empty()
                && stk.last().unwrap() <= cnt.first_key_value().map(|k| k.0).unwrap_or(&'z')
            {
                ret.push(stk.pop().unwrap());
            }
        }

        ret
    }
}

fn main() {}
