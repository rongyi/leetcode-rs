struct Solution;

impl Solution {
    pub fn video_stitching(mut clips: Vec<Vec<i32>>, time: i32) -> i32 {
        // 按起点排序
        clips.sort_unstable();

        let mut count = 0;
        let mut last = 0; // 当前覆盖到的边界
        let mut i = 0;
        let n = clips.len();

        while last < time {
            let mut max_reach = last;
            // 在所有能衔接当前边界的片段中，选能到最远的
            while i < n && clips[i][0] <= last {
                max_reach = max_reach.max(clips[i][1]);
                i += 1;
            }

            // 无法继续延伸
            if max_reach == last {
                return -1;
            }

            last = max_reach;
            count += 1;
        }

        count
    }
}

fn main() {}
