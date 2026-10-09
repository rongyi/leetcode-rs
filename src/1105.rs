struct Solution;

mod ai {
    struct Solution;
    impl Solution {
        pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
            let sz = books.len();
            let mut dp = vec![i32::MAX; sz + 1];
            dp[0] = 0;

            for i in 1..=sz {
                let mut acc_width = 0;
                let mut max_height = 0;
                for j in (0..i).rev() {
                    let (cur_width, cur_height) = (books[j][0], books[j][1]);
                    acc_width += cur_width;
                    if acc_width > shelf_width {
                        break;
                    }
                    max_height = max_height.max(cur_height);
                    dp[i] = dp[i].min(dp[j] + max_height);
                }
            }

            dp[sz]
        }
    }
}

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let sz = books.len();
        let mut dp = vec![i32::MAX; sz + 1];
        dp[0] = 0;

        for i in 0..sz {
            let mut width = 0;
            let mut height = 0;
            let mut j = i as i32;

            while j >= 0 && width + books[j as usize][0] <= shelf_width {
                width += books[j as usize][0];
                height = height.max(books[j as usize][1]);
                dp[i + 1] = dp[i + 1].min(dp[j as usize] + height);

                j -= 1;
            }
        }

        dp[sz]
    }
}

fn main() {}
