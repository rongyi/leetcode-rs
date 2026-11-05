struct Solution;

impl Solution {
    pub fn maximize_the_profit(n: i32, offers: Vec<Vec<i32>>) -> i32 {
        let sz = n as usize;
        let mut end_to_offers: Vec<Vec<(usize, i32)>> = vec![vec![]; sz];
        for offer in offers.iter() {
            let (start, end, gold) = (offer[0] as usize, offer[1] as usize, offer[2]);
            end_to_offers[end].push((start, gold));
        }
        // dp[i] max gold using houses 0 to i - 1
        let mut dp = vec![0; sz + 1];

        for i in 1..=sz {
            dp[i] = dp[i - 1];

            for &(start, gold) in end_to_offers[i - 1].iter() {
                dp[i] = dp[i].max(dp[start] + gold);
            }
        }

        dp[sz]
    }
}

fn main() {}
