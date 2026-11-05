
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut ret = Vec::new();
        let sz = 1 << n;

        for i in 0..sz {
            ret.push(i ^ (i >> 1));
        }

        ret
    }
}

