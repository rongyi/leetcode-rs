struct Solution;

impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut pg: Vec<(i32, i32)> = plant_time.into_iter().zip(grow_time.into_iter()).collect();
        // in grow time desending
        pg.sort_by(|a, b| b.1.cmp(&a.1));
        let mut total = 0;
        let mut cur_plant = 0;

        for i in 0..pg.len() {
            total = total.max(cur_plant + pg[i].0 + pg[i].1);
            cur_plant += pg[i].0;
        }

        total
    }
}

fn main() {}
