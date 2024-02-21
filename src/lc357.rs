struct Solution;

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }

        // 利用选择来做这道题
        // a[0] = 1
        // a[1] = 9 + a[0]
        // a[2] = 9*9 + a[1]
        // a[3] = 9*9*8 + a[2]
        // a[4] = 9*9*8*7 + a[3]
        // ...
        // a[n] = 9*9!/(10 - n)! + a[n - 1]
        // 第一位是9个可能选择，因为0不可以
        // 第二位是除了之前选的那个，同时又添加了0这个选择所以还是9个选择
        // 第三位就是前面两个排除之后的选择了，剩8个可能
        // and so on
        let mut ret = 10;
        let mut c = 1;
        for i in 1..n.min(10) {
            c = c * (10 - i);
            ret = ret + 9 * c;
        }

        ret
    }
}

fn main() {}
