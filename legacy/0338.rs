struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ret = Vec::with_capacity((n + 1) as usize);

        for i in 0..n + 1 {
            ret.push(i.count_ones() as i32);
        }


        ret
    }
}


fn main() {}
