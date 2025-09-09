struct Solution;

impl Solution {
    pub fn maximum_groups(grades: Vec<i32>) -> i32 {
        let sz = grades.len();
        let mut group_sz = 1;
        let mut total_group = 0;

        while total_group + group_sz <= sz {
            total_group += group_sz;
            group_sz += 1;
        }

        group_sz as i32 - 1
    }
}
fn main() {}
