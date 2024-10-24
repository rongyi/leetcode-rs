struct Solution;

impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        let digits: Vec<i32> = n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        let k = digits.len();

        let mut ret = 0;

        // count all uniq which is less than k wide
        for i in 1..k {
            // first digit can not select 0, so permutation is 9
            // after this, it's 9 * 8 * 7 * 6 ...
            ret += 9 * Self::perm(9, i as i32 - 1);
        }
        let mut used = vec![false; 10];
        // prefix
        // e.g. 8625, we calculate 1XXX -> 7XXX uniq
        //                         80XX -> 85XX
        for i in 0..k {
            for j in (if i > 0 { 0 } else { 1 })..digits[i] {
                if !used[j as usize] {
                    ret += Self::perm(10 - (i as i32 + 1), k as i32 - (i as i32 + 1));
                }
            }
            if used[digits[i] as usize] {
                break;
            }
            used[digits[i] as usize] = true;

            // when we come here we know this number n is also a uniq number
            if i == k - 1 {
                ret += 1;
            }
        }

        n - ret
    }

    fn perm(init: i32, cnt: i32) -> i32 {
        (0..cnt).fold(1, |mut acc, i| {
            acc *= init - i;
            acc
        })
    }
}

fn main() {
    let val = (0..0).fold(1, |mut acc, cur| {
        acc += cur;
        acc
    });
    println!("val{}", val);
}
