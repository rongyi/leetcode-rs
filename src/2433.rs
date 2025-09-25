struct Solution;

impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![pref[0]];

        for i in 1..pref.len() {
            ret.push(pref[i] ^ pref[i - 1]);
        }

        ret
    }
}

fn main() {
    let v = Solution::find_array(vec![5, 2, 0, 3, 1]);
    println!("{:?}", v);
}
