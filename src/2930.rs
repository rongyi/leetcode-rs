struct Solution;

impl Solution {
    pub fn string_count(n: i32) -> i32 {
        if n < 4 {
            return 0;
        } // Minimum length to have ee, l, t

        let n = n as i64;
        let m = 1_000_000_007;

        fn pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
            let mut res = 1;
            base %= m;
            while exp > 0 {
                if exp % 2 == 1 {
                    res = (res * base) % m;
                }
                base = (base * base) % m;
                exp /= 2;
            }
            res
        }

        // f(choices, n) calculates (choices^n + n * choices^(n-1))
        // This represents strings where 'e' appears 0 or 1 times.
        let f = |choices: i64, n: i64| -> i64 {
            if choices < 0 {
                return 0;
            }
            let term1 = pow(choices, n, m);
            let term2 = (n * pow(choices, n - 1, m)) % m;
            (term1 + term2) % m
        };

        // g(choices, n) is just choices^n
        // Used for sets B and C where a letter just cannot exist (0 times).
        let g = |choices: i64, n: i64| -> i64 {
            if choices < 0 {
                return 0;
            }
            pow(choices, n, m)
        };

        let total = pow(26, n, m);

        // Individual Sets
        let a = f(25, n); // < 2 'e's (25 choices for non-e slots)
        let b = g(25, n); // 0 'l's
        let c = g(25, n); // 0 't's

        // Intersections of two
        let ab = f(24, n); // < 2 'e's AND 0 'l's (24 choices)
        let ac = f(24, n); // < 2 'e's AND 0 't's (24 choices)
        let bc = g(24, n); // 0 'l's AND 0 't's (24 choices)

        // Intersection of all three
        let abc = f(23, n); // < 2 'e's AND 0 'l's AND 0 't's (23 choices)

        // PIE: |A| + |B| + |C| - |A∩B| - |A∩C| - |B∩C| + |A∩B∩C|
        let mut invalid = (a + b + c) % m;
        invalid = (invalid - ab + m) % m;
        invalid = (invalid - ac + m) % m;
        invalid = (invalid - bc + m) % m;
        invalid = (invalid + abc) % m;

        ((total - invalid + m) % m) as i32
    }
}

fn main() {}
