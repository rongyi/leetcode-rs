struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let sz = citations.len() as i32;
        let mut l: i32 = 0;
        let mut r: i32 = sz - 1;
        while l <= r {
            let mid = l + (r - l) / 2;
            let paper_cnt = sz - mid;
            if paper_cnt == citations[mid as usize] {
                return paper_cnt;
            } else if paper_cnt < citations[mid as usize] {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }

        sz - (r + 1)
    }
}

fn main() {
    unimplemented!();
}
