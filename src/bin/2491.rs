struct Solution;

impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let sz = skill.len();
        let mut skill: Vec<i64> = skill.into_iter().map(|v| v as i64).collect();
        skill.sort_unstable();

        let sum = skill[0] + skill[sz - 1];
        let mut chem = skill[0] * skill[sz - 1];
        for l in 1..sz / 2 {
            let r = sz - l - 1;
            if skill[l] + skill[r] == sum {
                chem += skill[l] * skill[r];
            } else {
                return -1;
            }
        }
        chem
    }
}

fn main() {}
