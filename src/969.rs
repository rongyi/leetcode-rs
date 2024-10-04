struct Solution;

impl Solution {
    pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::new();
        let sz = arr.len();

        for i in (1..=sz).rev() {
            let max_pos = arr[..i]
                .iter()
                .enumerate()
                .max_by_key(|&(_, &x)| x)
                .map(|(index, _)| index)
                .unwrap();

            // need swap
            if max_pos != i - 1 {
                // put max to front first
                if max_pos != 0 {
                    arr[..=max_pos].reverse();
                    ret.push((max_pos + 1) as i32);
                }
                // then swap entire to make max to its place
                arr[..i].reverse();
                ret.push(i as i32);
            }
        }

        ret
    }
}

fn main() {}
