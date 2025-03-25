#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn ways_to_split(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut prefix: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
        for i in 1..sz {
            prefix[i] += prefix[i - 1];
        }
        let sum = *prefix.last().unwrap();
        const MOD: i64 = 1e9 as i64 + 7;
        let mut third_section = 0i64;
        let mut ret: i64 = 0;
        for i in (2..sz).rev() {
            third_section += nums[i] as i64;

            // 3 sections sum as
            // a | b | c and a + b + c = sum
            // a | sum - (a + c) | c
            // a <= sum - (a + c)
            // sum-(a + c) <= c
            // make c as constant we get
            // sum - 2c  <= a <= (sum - c) / 2
            let cur = Self::count_range(
                &prefix,
                i - 1,
                sum - 2 * third_section,
                (sum - third_section) / 2,
            );
            ret = (ret + cur) % MOD;
        }

        ret as _
    }

    fn count_range(prefix: &[i64], end: usize, low: i64, high: i64) -> i64 {
        if low > high {
            return 0;
        }
        let l = prefix[0..end].partition_point(|&x| x < low);
        let r = prefix[0..end].partition_point(|&x| x <= high);
        (r - l) as i64
    }
}

fn main() {
    let a = vec![1, 2, 2, 2, 3, 4];
    // Demonstrating partition_point usage:
    // partition_point(|&x| x <= value) is equivalent to C++'s upper_bound(value)
    // partition_point(|&x| x < value) is equivalent to C++'s lower_bound(value)

    // upper_bound
    let pos = a.partition_point(|&x| x <= 2);
    println!("{pos}");
    // lower_bound
    let pos = a.partition_point(|&x| x < 2);
    println!("{pos}");
}
