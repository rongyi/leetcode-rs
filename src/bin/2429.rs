struct Solution;

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let one1 = Self::count_bit(num1);
        let one2 = Self::count_bit(num2);
        if one1 == one2 {
            return num1;
        }
        if one1 < one2 {
            println!("here");
            // add to end, to get minimized value with num1
            let mut add = one2 - one1;
            let mut ret = num1;
            for i in 0..32 {
                if add <= 0 {
                    break;
                }
                if num1 & (1 << i) == 0 {
                    ret |= 1 << i;
                    add -= 1;
                }
            }
            return ret;
        }
        // insert 1 from lower index
        let mut ret = num1;
        let mut delete = one1 - one2;

        for i in 0..32 {
            if delete <= 0 {
                break;
            }
            if num1 & (1 << i) != 0 {
                delete -= 1;
                ret &= !(1 << i);
            }
        }
        ret
    }
    fn count_bit(mut n: i32) -> i32 {
        let mut acc = 0;
        while n != 0 {
            acc += 1;
            n &= n - 1;
        }

        acc
    }
}

fn main() {
    let v = Solution::minimize_xor(1, 12);
    println!("{v}");
}
