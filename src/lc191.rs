struct Solution;

impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        let mut n = n;
        let mut cnt = 0;

        while n != 0 {
            cnt += 1;
            n = n & (n - 1);
        }


        cnt
    }
}



fn main() {}
