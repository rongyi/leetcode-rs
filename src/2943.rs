struct Solution;

impl Solution {
    pub fn maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        let mut h_bars = h_bars;
        let mut v_bars = v_bars;
        h_bars.sort();
        v_bars.sort();

        let max_h_gap = Self::max_consecutive(&h_bars);
        let max_v_gap = Self::max_consecutive(&v_bars);
        let side = max_h_gap.min(max_v_gap) + 1;
        (side * side) as i32
    }

    fn max_consecutive(bars: &[i32]) -> usize {
        let mut max_len = 1;
        let mut cur_len = 1;
        for i in 1..bars.len() {
            if bars[i] == bars[i - 1] + 1 {
                cur_len += 1;
                max_len = max_len.max(cur_len);
            } else {
                cur_len = 1;
            }
        }
        max_len
    }
}

fn main() {}
