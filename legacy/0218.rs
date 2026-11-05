struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut edges: Vec<(i32, i32)> = Vec::new();

        for i in 0..buildings.len() {
            let left = buildings[i][0];
            let right = buildings[i][1];
            let height = buildings[i][2];

            edges.push((left, -height));
            edges.push((right, height));
        }
        edges.sort();

        let mut ret: Vec<Vec<i32>> = Vec::new();
        // rust don't have multiset, we use btreemap to mimic
        let mut cnt: BTreeMap<i32, i32> = BTreeMap::new();
        cnt.insert(0, 1);
        let mut prev = 0;

        for i in 0..edges.len() {
            let (pos, height) = edges[i];
            if height < 0 {
                // println!("push {}", -height);
                let e = cnt.entry(-height).or_default();
                *e += 1;
            } else {
                // println!("pop {}", height);
                let e = cnt.get_mut(&height).unwrap();
                *e -= 1;
                if *e == 0 {
                    cnt.remove(&height);
                }
            }
            let (&cur, _) = cnt.iter().rev().next().unwrap();
            // println!("prev: {}, cur height: {}", prev, cur);
            if cur != prev {
                // println!("so we get height {} at postion: {}", cur, pos);
                ret.push(vec![pos, cur]);
                prev = cur;
            }
            // else {
            //     println!("cur height {} still the heighest, shaow all other height", cur);
            // }
        }

        ret
    }
}

fn main() {
    let buildings = vec![
        vec![2, 9, 10],
        vec![3, 7, 15],
        vec![5, 12, 12],
        vec![15, 20, 10],
        vec![19, 24, 8],
    ];
    Solution::get_skyline(buildings);
}
