struct Solution;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let sz = nums.len();
        // 1 based
        let mut bit = vec![0; sz + 1];
        let rank = Self::get_rank(&nums);
        let mut ret = vec![0; sz];

        // not related with number value, just rank
        for i in (0..sz).rev() {
            let r = rank[i];
            // query rank lower than current
            ret[i] = Self::query(&bit, r - 1);
            // update r one more
            Self::update(&mut bit, r, 1);
        }

        ret
    }
    fn query(bit: &Vec<i32>, mut idx: usize) -> i32 {
        let mut acc = 0;
        while idx > 0 {
            acc += bit[idx];
            idx -= idx & (!idx + 1);
        }
        acc
    }
    fn update(bit: &mut Vec<i32>, mut idx: usize, diff: i32) {
        let sz = bit.len();
        while idx < sz {
            bit[idx] += diff;
            idx += idx & (!idx + 1);
        }
    }
    fn get_rank(nums: &[i32]) -> Vec<usize> {
        let mut sorted: Vec<_> = nums.iter().collect();
        sorted.sort();
        sorted.dedup();

        nums.iter()
            .map(|x| sorted.binary_search(&x).unwrap() + 1)
            .collect()
    }
}

fn main() {}
