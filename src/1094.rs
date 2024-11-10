struct Solution;

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut stops = vec![0; 1001];
        for t in trips.iter() {
            stops[t[1] as usize] += t[0];
            stops[t[2] as usize] -= t[0];
        }
        let mut people = 0;
        for i in 0..1001 {
            if people + stops[i] > capacity {
                return false;
            }
            people += stops[i];
        }
        true
    }
}
fn main() {}
