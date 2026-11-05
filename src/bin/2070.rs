struct Solution;

impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let sz = queries.len();
        let mut query_with_index: Vec<(i32, usize)> = queries
            .into_iter()
            .enumerate()
            .map(|(idx, val)| (val, idx))
            .collect();
        query_with_index.sort_by_key(|x| x.0);

        // by price
        items.sort_by_key(|x| x[0]);
        let mut ret = vec![0; sz];

        let mut i = 0;
        let mut max_beauty = 0;
        for &(val, idx) in query_with_index.iter() {
            while i < items.len() && items[i][0] <= val {
                max_beauty = max_beauty.max(items[i][1]);
                i += 1;
            }
            ret[idx] = max_beauty;
        }

        ret
    }
}

fn main() {}
