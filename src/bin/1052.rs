struct Solution;

impl Solution {
    pub fn max_satisfied(mut customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let sz = customers.len();
        let mut directly_satisfied = 0;
        for i in 0..sz {
            if grumpy[i] == 0 {
                directly_satisfied += customers[i];
                customers[i] = 0;
            }
        }

        let mut cur_sum = 0;
        let mut secret_satisfied = 0;
        // sliding window
        for i in 0..sz {
            cur_sum += customers[i];
            if i as i32 >= minutes {
                cur_sum -= customers[i - minutes as usize];
            }

            secret_satisfied = secret_satisfied.max(cur_sum);
        }

        directly_satisfied + secret_satisfied
    }
}

fn main() {}
