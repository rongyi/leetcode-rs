struct Solution;

impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        let mut nums = vec![n];
        let mut cur = n - 1;
        let mut index = 0;
        while cur > 0 {
            match index % 4 {
                0 => {
                    let val = nums.pop().unwrap() * cur;
                    nums.push(val);
                }
                1 => {
                    let val = nums.pop().unwrap() / cur;
                    nums.push(val);
                }
                2 => nums.push(cur),
                3 => nums.push(-cur),
                _ => unreachable!(),
            }
            index += 1;

            cur -= 1;
        }

        nums.into_iter().sum()
    }
}

fn main() {}
