#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn get_max_grid_happiness(
        m: i32,
        n: i32,
        introverts_count: i32,
        extroverts_count: i32,
    ) -> i32 {
        // m/n最大5，内向外向最多6, prev最长3^5 == 243
        // 怎么来的呢？只要最多前五个状态即可。每一个格子有三个状态，不放、放内向的、放外向的
        // 五个盒子，每个盒子三个状态，cache这么多就可以了，紧挨着的之前的是左边，前五个是上面
        //           N
        //         「 」
        // 0: empty
        // 1: put intro
        // 2: put extro
        let mut dp = vec![vec![vec![vec![vec![-1; 243]; 7]; 7]; 6]; 6];

        Self::recur(
            &mut dp,
            m as usize,
            n as usize,
            0,
            0,
            introverts_count,
            extroverts_count,
            0,
        )
    }

    fn recur(
        dp: &mut Vec<Vec<Vec<Vec<Vec<i32>>>>>,
        m: usize,
        n: usize,
        mut x: usize,
        mut y: usize,
        icount: i32,
        ecount: i32,
        mut prevn: i32,
    ) -> i32 {
        if y == n {
            y = 0;
            x += 1;
        }
        if icount == 0 && ecount == 0 {
            return 0;
        }
        if x == m {
            return 0;
        }
        if dp[x][y][icount as usize][ecount as usize][prevn as usize] != -1 {
            return dp[x][y][icount as usize][ecount as usize][prevn as usize];
        }
        // 1. leave current cel empty
        let mut ret = Self::recur(
            dp,
            m,
            n,
            x,
            y + 1,
            icount,
            ecount,
            Self::shift_with(prevn, 0),
        );
        let up = Self::get_pick(prevn, n - 1);
        let left = Self::get_pick(prevn, 0);
        if icount > 0 {
            let tmp = prevn;
            // put 1 in here
            prevn = Self::shift_with(prevn, 1);
            let mut add = 120;

            if x > 0 && up != 0 {
                add -= 30; // 内向遇到人了
                if up == 1 {
                    add -= 30; // 对方也是内向
                } else {
                    add += 20; // 对方外向
                }
            }
            if y > 0 && left != 0 {
                add -= 30;
                if left == 1 {
                    add -= 30; // 对方也是内向
                } else {
                    add += 20; // 对方外向
                }
            }
            ret = ret.max(Self::recur(dp, m, n, x, y + 1, icount - 1, ecount, prevn) + add);

            prevn = tmp;
        }
        if ecount > 0 {
            let tmp = prevn;
            prevn = Self::shift_with(prevn, 2);
            let mut add = 40;

            if x > 0 && up != 0 {
                add += 20;
                if up == 1 {
                    add -= 30;
                } else {
                    add += 20;
                }
            }
            if y > 0 && left != 0 {
                add += 20;
                if left == 1 {
                    add -= 30;
                } else {
                    add += 20;
                }
            }
            ret = ret.max(Self::recur(dp, m, n, x, y + 1, icount, ecount - 1, prevn) + add);

            prevn = tmp;
        }
        dp[x][y][icount as usize][ecount as usize][prevn as usize] = ret;

        ret
    }

    fn shift_with(prev: i32, cur: i32) -> i32 {
        (prev * 3 + cur) % 243
    }
    fn get_pick(mut mask: i32, shift: usize) -> i32 {
        mask /= 3i32.pow(shift as u32);
        mask % 3
    }
}

fn main() {}
