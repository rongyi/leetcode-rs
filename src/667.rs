struct Solution;

impl Solution {
    pub fn construct_array(n: i32, mut k: i32) -> Vec<i32> {
        let mut ret = Vec::new();
        let mut i = 1;
        let mut j = n;

        while i <= j {
            if k > 1 {
                let val = if k % 2 == 1 {
                    let v = i;
                    i += 1;
                    v
                } else {
                    let v = j;
                    j -= 1;
                    v
                };
                k -= 1;
                ret.push(val);
            } else {
                ret.push(i);
                i += 1;
            }
        }

        ret
    }
}

fn main() {}
