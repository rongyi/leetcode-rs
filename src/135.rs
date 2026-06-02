struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let sz = ratings.len();
        let mut dist: Vec<i32> = vec![1; sz];

        for i in 1..sz {
            if ratings[i] > ratings[i - 1] {
                dist[i] = dist[i - 1] + 1;
            }
        }

        for i in (0..sz - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                dist[i] = dist[i].max(dist[i + 1] + 1);
            }
        }

        dist.into_iter().sum()
    }
}

fn main() {}
