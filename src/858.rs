struct Solution;

impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let mut k = 1;
        while p * k % q != 0 {
            k += 1;
        }
        if k & 1 != 0 {
            let d = p * k / q;
            if d & 1 != 0 {
                return 1;
            }
            return 2;
        }

        return 0;
    }
}

fn main() {}
