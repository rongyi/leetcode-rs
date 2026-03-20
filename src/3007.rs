struct Solution;

impl Solution {
    pub fn find_maximum_number(k: i64, x: i32) -> i64 {
        let mut low = 1i64;
        let mut high = 1e16 as i64; // Sufficiently large upper bound
        let mut ans = 1;

        while low <= high {
            let mid = low + (high - low) / 2;
            if self::get_total_price(mid, x) <= k {
                ans = mid;
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        ans
    }
}

fn get_total_price(n: i64, x: i32) -> i64 {
    let mut total_price = 0;
    // We check bits at 1-indexed positions i such that i % x == 0
    // In 0-indexing, these are (i-1), so positions x-1, 2x-1, 3x-1...
    for i in (x..63).step_by(x as usize) {
        total_price += count_set_bits_at_position(n, (i - 1) as u32);
    }
    total_price
}

fn count_set_bits_at_position(n: i64, i: u32) -> i64 {
    let period = 1i64 << (i + 1);
    let full_periods = (n + 1) / period;
    let mut count = full_periods * (1i64 << i);

    let remainder = (n + 1) % period;
    if remainder > (1i64 << i) {
        count += remainder - (1i64 << i);
    }
    count
}

fn main() {}
