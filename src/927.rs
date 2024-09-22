struct Solution;

impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        // 1 index
        let dp: Vec<usize> = arr
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1)
            .map(|(i, _)| i)
            .collect();

        // can not dive 1
        if dp.len() % 3 != 0 {
            return vec![-1, -1];
        }
        // no 1
        if dp.is_empty() {
            return vec![0, 2];
        }
        // let l1 = 0;
        let l2 = dp.len() / 3;
        let l3 = l2 * 2;

        for i in 1..l2 {
            let diff = dp[i] - dp[i - 1];
            if dp[l2 + i] - dp[l2 + i - 1] != diff || dp[l3 + i] - dp[l3 + i - 1] != diff {
                return vec![-1, -1];
            }
        }
        // 后面的0个数也要一样，前面可以不一样
        // 算0，这个厉害
        // 如果最后一位是1的话那么这个tail0是1，所以
        // 这里大家都没有减一
        let tail0 = arr.len() - *dp.last().unwrap();
        // 以l3举例
        // l3是第三部分的开始的第一个1， l3 - 1是第二部分的最后一个1，之间的gap就是中间有多少个0 - 1
        // 而对标的tail0也没有减一所以大家都这么算了
        // 另外为什么是小于，因为多了可以切，少了补不了啊
        if dp[l3] - dp[l3 - 1] < tail0 || dp[l2] - dp[l2 - 1] < tail0 {
            return vec![-1, -1];
        }

        vec![(dp[l2 - 1] + tail0 - 1) as i32, (dp[l3 - 1] + tail0) as i32]
    }
}

fn main() {}
