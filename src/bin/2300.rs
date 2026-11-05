struct Solution;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut potions: Vec<i64> = potions.into_iter().map(|x| x as i64).collect();
        potions.sort_unstable();
        let n = spells.len();
        let m = potions.len();
        let mut ret = vec![0; n];

        for i in 0..n {
            let val = spells[i] as i64;
            let min_product = (success + val - 1) / val;
            // lower_bound of min_product
            let idx = potions.partition_point(|x| *x < min_product);
            ret[i] = (m - idx) as i32;
        }

        ret
    }
}

fn main() {}
