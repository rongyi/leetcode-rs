struct Solution;

// suck problem
impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        // number with 1 digits: 1,2,3,..,9 ==>9 in total
        // number with 2 digits: 10,11,...,99 ==> 90 in total
        // number with 3 digits: 100,101,...,999 ==> 900 in total
        // .....
        // number with b digits: 10^b,10^b+1,...,10^(b+1)-1 ==> 9*10^b in total

        // a is the number of digits for that number
        let mut a: i64 = 1;
        // b is the power of one number
        let mut b: i64 = 0;
        // sum record the total number digits
        let mut sum: i64 = 0;
        let n = n as i64;

        sum += a * (9 * 10i64.pow(b as u32));

        while sum < n {
            a += 1;
            b += 1;

            sum += a * (9 * 10i64.pow(b as u32));
        }

        // We need to deduce the last part
        sum -= a * (9 * 10i64.pow(b as u32));

        // 找到对应那个数 ，应该是从  n - (sum + 1) 推导过来, sum + 1 调到第一个 a位数上来。 n减从而算出了offset。
        // a == 2时
        // 6 7 8 9  1   0  1  1  1  2 1  3 序列
        // 6 7 8 9  10  11 12 13 14 15...  n的值
        // (n - (sum + 1) ) / a ==> 得到落在那个数字上
        // (n - (sum + 1) ) % a ==> 得到该位数字的index上。

        let dig = (n - sum - 1) / a;
        let bit = (n - sum - 1) % a;
        let num = 10i64.pow(b as u32) + dig;
        let s: Vec<char> = num.to_string().chars().collect();

        s[bit as usize] as i32 - '0' as i32
    }
}

fn main() {
    println!("{}", 10i32.pow(2));
}
