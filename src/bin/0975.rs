struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn odd_even_jumps(arr: Vec<i32>) -> i32 {
        let sz = arr.len();
        let mut odd = vec![false; sz];
        let mut even = vec![false; sz];
        odd[sz - 1] = true;
        even[sz - 1] = true;

        // last one is always ok
        let mut acc = 1;

        let mut index: BTreeMap<i32, usize> = BTreeMap::new();
        // put last in
        index.insert(arr[sz - 1], sz - 1);

        for i in (0..sz - 1).rev() {
            // odd jump select min in >= cur
            if let Some((_, &idx)) = index.range(arr[i]..).next() {
                odd[i] = even[idx];
            }
            // even jump select max in <= cur
            if let Some((_, &idx)) = index.range(..=arr[i]).next() {
                even[i] = odd[idx];
            }
            if odd[i] {
                acc += 1;
            }
            index.insert(arr[i], i);
        }

        acc
    }
}

fn main() {}
