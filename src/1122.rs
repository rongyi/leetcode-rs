struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut val_index: HashMap<i32, usize> = HashMap::new();
        for (i, &val) in arr2.iter().enumerate() {
            val_index.insert(val, i);
        }
        let mut have_index_vals: Vec<i32> = arr1
            .iter()
            .filter(|&x| val_index.contains_key(&x))
            .copied()
            .collect();
        let mut no_index_vals: Vec<i32> = arr1
            .iter()
            .filter(|&x| !val_index.contains_key(&x))
            .copied()
            .collect();
        have_index_vals.sort_by(|l, r| val_index[l].cmp(&val_index[r]));
        no_index_vals.sort_unstable();
        have_index_vals.extend(&no_index_vals[..]);

        have_index_vals
    }
}

fn main() {}
