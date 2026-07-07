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

        sum -= a * (9 * 10i64.pow(b as u32));

        let dig = (n - sum - 1) / a;
        let bit = (n - sum - 1) % a;
        let num = 10i64.pow(b as u32) + dig;
        let s: Vec<char> = num.to_string().chars().collect();

        s[bit as usize] as i32 - '0' as i32
    }
}

fn main() {}
