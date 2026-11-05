struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut ret = Vec::new();
        for h in 0..12i32 {
            for m in 0..60i32 {
                if h.count_ones() + m.count_ones() == turned_on as u32 {
                    ret.push(format!("{}:{:02}", h, m));
                }
            }
        }

        ret
    }
}

fn main() {
    println!("{}", 10i32.pow(2));
}
