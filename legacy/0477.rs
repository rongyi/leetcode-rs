struct Solution;

impl Solution {
    // 全局的看每一位，如果该位上0的个数有n个，1的个数有q个，那么对结果的贡献就是 p * q
    // 为什么？因为通过TLE的版本可以看到碰撞的方式就是每个0会和每个1碰撞一次并且产生结果
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        let total = nums.len() as i32;
        for i in 0..32 {
            let mut one_count = 0;
            for &num in &nums {
                one_count += (num >> i) & 1;
            }
            ret += one_count * (total - one_count);
        }
        ret
    }
}

fn main() {}
