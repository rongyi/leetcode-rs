struct Solution;

impl Solution {
    pub fn arrange_coins(mut n: i32) -> i32 {
        let mut complete_row = 0;
        let mut row_num = 1;
        while n >= row_num {
            complete_row += 1;
            n -= row_num;
            row_num += 1;
        }

        complete_row
    }
}

fn main() {}
