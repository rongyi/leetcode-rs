struct Solution;

mod ai {
    struct Solution;

    impl Solution {
        pub fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
            let t = time as usize;
            let mut dp = vec![i32::MAX; t + 1];
            dp[0] = 0;

            for i in 0..=t {
                if dp[i] == i32::MAX {
                    continue;
                }
                for clip in &clips {
                    let (s, e) = (clip[0] as usize, (clip[1] as usize).min(t));
                    // i < e 是方便延展，可以拓出去一些
                    if s <= i && i < e {
                        // 用这个片段覆盖 [i, e]
                        for j in i..=e {
                            dp[j] = dp[j].min(dp[i] + 1);
                        }
                    }
                }
            }

            if dp[t] == i32::MAX {
                -1
            } else {
                dp[t]
            }
        }
    }
}

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
