struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort();
        let mut ret = 0;
        let mut prev = 0;
        for i in 1..intervals.len() {
            // oh shit, we are covered by prev range!
            if intervals[i][0] < intervals[prev][1] {
                ret += 1;
                // 拿掉横跨最靠前的那个罩子，此时prev指向当前
                // 当前的区间完全被cover了，删掉最靠前的那个，此时prev有变化
                // prev指向当前这个区间
                if intervals[i][1] < intervals[prev][1] {
                    prev = i;
                }
                // 最靠前的区间就是当前区间，我们干掉这个罩子，prev没有变化
            } else {
                prev = i;
            }
        }

        ret
    }
}

fn main() {}
