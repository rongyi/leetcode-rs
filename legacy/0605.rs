struct Solution;

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut cnt = 0;
        let mut i = 0;
        // just put there
        while i < flowerbed.len() {
            if flowerbed[i] == 0 {
                if (i == 0 || flowerbed[i - 1] == 0)
                    && (i == flowerbed.len() - 1 || flowerbed[i + 1] == 0)
                {
                    cnt += 1;
                    flowerbed[i] = 1;
                }
            }

            i += 1;
        }
        cnt >= n
    }
}

fn main() {}
