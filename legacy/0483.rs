struct Solution;

impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        let num = n.parse::<u64>().unwrap();

        let mut len = ((num + 1) as f64).log2() as i64;
        while len >= 2 {
            let mut l: u64 = 2;
            let mut r = (f64::powf(num as f64, 1.0 / (len - 1) as f64) + 1.0) as u64;

            while l < r {
                let mut sum = 0;
                let base = l + (r - l) / 2;
                let mut val = 1;
                for _i in 0..len {
                    sum += val;
                    val *= base;
                }
                if sum == num {
                    return base.to_string();
                } else if sum < num {
                    l = base + 1;
                } else {
                    r = base;
                }
            }
            len -= 1;
        }

        return (num - 1).to_string();
    }
}

fn main() {}
