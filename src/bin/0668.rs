struct Solution;

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let mut lo = 1;
        let mut hi = m * n;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let mut cnt = 0;
            let mut j = n;

            // scan row find value <= mid count and then accumulate
            for i in 1..=m {
                while j >= 1 && i * j > mid {
                    j -= 1;
                }
                // i, j is not index but value, so here we plus j, not j - 1
                cnt += j;
            }

            // 一般二叉查找树找到会立即返回，这里没有，这里还是一直碾下去
            // 为什么？因为可能停留的值不在乘法表里

            // https://leetcode.com/problems/kth-smallest-number-in-multiplication-table/discuss/262279/Python-Binary-Search-Need-to-Return-the-Smallest-Candidate
            // So why the smallest candidate is in M table?
            // Because if the smallest candidate(no smaller than k numbers in M
            // table), saying x, is not in M table, then x-1 will also be a
            // candidate(no smaller than k numbers in M table) since x is not in the
            // table. Then x is not the smallest candidate.
            // 反证法证明出来的
            if cnt < k {
                lo = mid + 1;
            } else {
                // keep shrink, not return even for cnt == k
                hi = mid;
            }
        }

        lo
    }
}

fn main() {}
