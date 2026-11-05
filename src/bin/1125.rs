struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let sz = req_skills.len();
        let mut ret: HashMap<i32, Vec<i32>> = HashMap::new();
        ret.insert(0, Vec::new());
        ret.reserve(1 << sz);

        // label skill as each one as a bit postion
        // the or operation below
        let mut skill_idx: HashMap<String, i32> = HashMap::new();
        for (i, skill) in req_skills.iter().enumerate() {
            skill_idx.insert(skill.clone(), i as i32);
        }

        for i in 0..people.len() {
            let mut cur_skill = 0;

            for skill in people[i].iter() {
                cur_skill |= 1 << *skill_idx.get(skill).unwrap();
            }

            let mut ret_cp = ret.clone();

            for (&cur_mask, cur_ps) in ret.iter() {
                let comb = cur_mask | cur_skill;
                if !ret_cp.contains_key(&comb)
                    || ret_cp.get(&comb).unwrap().len() > 1 + cur_ps.len()
                {
                    let mut new_comb_peoples = cur_ps.clone();
                    new_comb_peoples.push(i as i32);
                    ret_cp.insert(comb, new_comb_peoples);
                }
            }
            ret = ret_cp;
        }

        ret.remove(&((1 << sz) - 1)).unwrap()
    }
}

fn main() {}
