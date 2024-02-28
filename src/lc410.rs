struct Solution;

impl Solution {
    // binary search way
    // 思路：
    // 调整max，这个max有个区间，即[min(nums), sum(nums)]
    // 在逐渐调整过程中看用m - 1 刀切下去是否满足条件，能满足
    // 就再往下压一直压到第一个满足的即可
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let nums: Vec<i64> = nums.iter().map(|&n| n as i64).collect();
        let mut left = *nums.iter().min().unwrap();
        let mut right = nums.iter().fold(0i64, |acc, &cur| acc + cur);
        while left < right {
            let mid = left + (right - left) / 2;
            if Self::cansplit(&nums, k - 1, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left as i32
    }

    // 下手cuts刀之后把数组切割为 cuts + 1块
    // 每一块的和的上限是max
    // 在尽可能在满足一块之和 <= max这种情况。
    // 这样算下来的刀数就是最少了，看看这种情况是否满足？
    // 如果发现单独一个元素的都比max还大，那完了，切最多的刀留最小的块都不满足
    fn cansplit(nums: &[i64], mut cut: i32, max: i64) -> bool {
        let mut acc = 0;
        for &num in nums {
            if num > max {
                return false;
            }
            if acc + num <= max {
                acc += num;
            } else {
                cut -= 1;
                acc = num;
                if cut < 0 {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    println!("{}", 10i32.pow(2));
}
