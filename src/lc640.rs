struct Solution;

impl Solution {
    pub fn solve_equation(equation: String) -> String {
        let mut chunks: Vec<Vec<char>> = Vec::new();
        for chunk in equation.split('=') {
            chunks.push(chunk.chars().collect());
        }
        // add meaning less '+' to eliminate process
        for c in chunks.iter_mut() {
            c.push('+');
        }

        let (left_coff, left_constant) = Self::reduce(&chunks[0]);
        let (right_coff, right_constant) = Self::reduce(&chunks[1]);
        let coff = left_coff - right_coff;
        let constants = right_constant - left_constant;
        if coff == 0 {
            if constants == 0 {
                return "Infinite solutions".to_string();
            }
            return "No solution".to_string();
        }

        format!("x={}", constants / coff)
    }

    // coefficient, constant
    fn reduce(eqs: &Vec<char>) -> (i32, i32) {
        let mut prev_cache: String = String::new();
        let mut co_sum = 0;
        let mut con_sum = 0;

        fn parse_coefficient(s: &str) -> i32 {
            if s == "+x" || s == "x" {
                return 1;
            } else if s == "-x" {
                return -1;
            }

            println!("{}", s);
            return s[0..s.len() - 1].parse().unwrap();
        }

        for &c in eqs.iter() {
            match c {
                '+' | '-' => {
                    if prev_cache.ends_with('x') {
                        co_sum += parse_coefficient(&prev_cache);
                    } else {
                        // for empty case
                        con_sum += prev_cache.parse::<i32>().unwrap_or(0);
                    }
                    prev_cache.clear();

                    // for next round
                    prev_cache.push(c);
                }
                _ => {
                    prev_cache.push(c);
                }
            }
        }

        (co_sum, con_sum)
    }
}

fn main() {
    let ret = Solution::solve_equation("-x=-1".to_string());
    println!("{}", ret);
}
