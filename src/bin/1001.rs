
struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut x: HashMap<i32, i32> = HashMap::new();
        let mut y: HashMap<i32, i32> = HashMap::new();
        let mut diag: HashMap<i32, i32> = HashMap::new();
        let mut anti_diag: HashMap<i32, i32> = HashMap::new();

        let mut ls: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut ret = Vec::new();

        for l in lamps.iter() {
            let (i, j) = (l[0], l[1]);
            if ls.entry(i).or_insert(HashSet::new()).insert(j) {
                *x.entry(i).or_insert(0) += 1;
                *y.entry(j).or_insert(0) += 1;
                *diag.entry(i + j).or_insert(0) += 1;
                *anti_diag.entry(i - j).or_insert(0) += 1;
            }
        }
        for q in queries.iter() {
            let (i, j) = (q[0], q[1]);
            if *x.get(&i).unwrap_or(&0) > 0
                || *y.get(&j).unwrap_or(&0) > 0
                || *diag.get(&(i + j)).unwrap_or(&0) > 0
                || *anti_diag.get(&(i - j)).unwrap_or(&0) > 0
            {
                ret.push(1);
                for li in i - 1..=i + 1 {
                    for lj in j - 1..=j + 1 {
                        if let Some(v) = ls.get_mut(&li) {
                            if v.contains(&lj) {
                                v.remove(&lj);
                                x.entry(li).and_modify(|v| *v -= 1);
                                y.entry(lj).and_modify(|v| *v -= 1);
                                diag.entry(li + lj).and_modify(|v| *v -= 1);
                                anti_diag.entry(li - lj).and_modify(|v| *v -= 1);
                            }
                        }
                    }
                }
            } else {
                ret.push(0);
            }
        }

        ret
    }
}

fn main() {}
