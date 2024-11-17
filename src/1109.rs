struct Solution;

impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut ret = vec![0; n as usize];
        for b in bookings.into_iter() {
            let (start, end, val) = (b[0], b[1], b[2]);
            ret[(start - 1) as usize] += val;

            if end < n {
                ret[end as usize] -= val;
            }
        }
        for i in 1..ret.len() {
            ret[i] += ret[i - 1];
        }
        ret
    }
}

fn main() {}
