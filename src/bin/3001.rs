struct Solution;

impl Solution {
    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        // 1. Check Rook (a, b) to Queen (e, f)
        if a == e {
            // Rook and Queen in same row. Is Bishop blocking?
            if c == a && ((d > b && d < f) || (d > f && d < b)) {
                // Blocked
            } else {
                return 1;
            }
        }
        if b == f {
            // Rook and Queen in same column. Is Bishop blocking?
            if d == b && ((c > a && c < e) || (c > e && c < a)) {
                // Blocked
            } else {
                return 1;
            }
        }

        // 2. Check Bishop (c, d) to Queen (e, f)
        if (c - e).abs() == (d - f).abs() {
            // Bishop and Queen on same diagonal. Is Rook blocking?
            if (c - a).abs() == (d - b).abs() && (a - e).abs() == (b - f).abs() {
                // Rook is on the same diagonal. Is it between them?
                if ((c < a && a < e) || (e < a && a < c)) && ((d < b && b < f) || (f < b && b < d))
                {
                    // Blocked
                } else {
                    return 1;
                }
            } else {
                return 1;
            }
        }

        // 3. If no 1-move capture is possible, it's always 2
        2
    }
}

fn main() {}
