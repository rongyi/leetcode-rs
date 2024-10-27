struct Solution;

impl Solution {
    pub fn video_stitching(mut clips: Vec<Vec<i32>>, time: i32) -> i32 {
        clips.sort_unstable();
        let mut cur_end = 0;
        let mut ret = 0;
        let mut prev_end = -1;
        for v in clips.iter() {
            if v[1] <= cur_end {
                continue;
            }
            if v[0] > cur_end {
                return -1;
            }
            if v[0] > prev_end {
                prev_end = cur_end;
                ret += 1;
            }
            cur_end = v[1];
            if cur_end >= time {
                return ret;
            }
        }

        -1
    }
}

fn main() {}
