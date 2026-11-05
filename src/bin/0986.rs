struct Solution;

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let m = first_list.len();
        let n = second_list.len();
        let mut i = 0;
        let mut j = 0;
        let mut ret = Vec::new();
        while i < m && j < n {
            let cura = &first_list[i];
            let curb = &second_list[j];

            let start = cura[0].max(curb[0]);
            let end = cura[1].min(curb[1]);
            if start <= end {
                ret.push(vec![start, end]);
            }
            if cura[1] == curb[1] {
                i += 1;
                j += 1;
            } else if cura[1] > curb[1] {
                j += 1;
            } else {
                i += 1;
            }
        }

        ret
    }
}

fn main() {}
