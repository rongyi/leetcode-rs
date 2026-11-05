struct Solution;

impl Solution {
    // For people who doesn't understand the author's explanations, just look at some examples:
    //
    // Let n = 4560000
    //
    // How many nums with "1" at the thousand's position?
    //
    // 4551000 to 4551999 (1000)
    // 4541000 to 4541999 (1000)
    // 4531000 to 4531999 (1000)
    // ...
    // 1000 to 1999 (1000)
    //
    // That's 456 * 1000
    //
    // What if n = 4561234?
    //
    // 4561000-4561234 (1234+1)
    // 4551000 to 4551999 (1000)
    // 4541000 to 4541999 (1000)
    // 4531000 to 4531999 (1000)
    // ...
    // 1000 to 1999 (1000)
    //
    // That's 456 * 1000 + 1234 + 1
    //
    // What if n = 4562345?
    // 4561000-4561999 (1000)
    // 4551000 to 4551999 (1000)
    // 4541000 to 4541999 (1000)
    // 4531000 to 4531999 (1000)
    // ...
    // 1000 to 1999 (1000)
    //
    // That's 456*1000 + 1000

    pub fn count_digit_one(n: i32) -> i32 {
        let mut count = 0;
        let mut factor = 1;
        let n = n as i64;
        let mut cur = n / factor;

        while cur > 0 {
            let r = cur % 10;
            let q = cur / 10;

            count += q * factor;

            if r == 1 {
                count += n % factor + 1;
            } else if r > 1 {
                count += factor;
            }

            factor *= 10;
            cur = n / factor;
        }
        count as i32
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

fn main() {}
