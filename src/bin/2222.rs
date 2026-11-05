struct Solution;

impl Solution {
    pub fn number_of_ways(s: String) -> i64 {
        let s = s.as_bytes();
        let sz = s.len();
        let mut one_left_zero = vec![0; sz];
        let mut one_right_zero = vec![0; sz];

        let mut zero_left_one = vec![0; sz];
        let mut zero_right_one = vec![0; sz];

        let mut zero_acc = 0;
        let mut one_acc = 0;
        for i in 0..sz {
            one_left_zero[i] = zero_acc;
            zero_left_one[i] = one_acc;

            if s[i] == b'0' {
                zero_acc += 1;
            } else {
                one_acc += 1;
            }
        }
        zero_acc = 0;
        one_acc = 0;
        for i in (0..sz).rev() {
            one_right_zero[i] = zero_acc;
            zero_right_one[i] = one_acc;

            if s[i] == b'0' {
                zero_acc += 1;
            } else {
                one_acc += 1;
            }
        }
        let mut total = 0;

        for i in 0..sz {
            // form 101, find left and right one
            if s[i] == b'0' {
                let l = zero_left_one[i];
                let r = zero_right_one[i];
                total += l * r;
            } else {
                // form 010
                let l = one_left_zero[i];
                let r = one_right_zero[i];
                total += l * r;
            }
        }

        total
    }
}

fn main() {}
