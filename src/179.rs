struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut num_strs: Vec<String> = nums.iter().map(|n| n.to_string()).collect();
        num_strs.sort_by(|a, b| {
            let mut o1 = a.clone();
            o1.push_str(b);
            let mut o2 = b.clone();
            o2.push_str(a);

            o2.cmp(&o1)
        });
        println!("{:?}", num_strs);
        let ret = num_strs.join("");
        if ret.starts_with('0') && ret.len() > 1 {
            return "0".to_string();
        }
        ret
    }
}

fn main() {
    let input = vec![3, 30, 34, 5, 9];
    let a = Solution::largest_number(input);
    println!("{}", a);
}
