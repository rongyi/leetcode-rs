struct Solution;

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let (a, b) = Self::parse_complex(&num1);
        let (c, d) = Self::parse_complex(&num2);

        let real = a * c - b * d;
        let img = b * c + a * d;

        format!("{}+{}i", real, img)
    }

    fn parse_complex(num: &str) -> (i32, i32) {
        let parts: Vec<&str> = num.split('+').collect();
        let real = parts[0].parse::<i32>().unwrap();
        let imaginary = parts[1].trim_end_matches('i').parse::<i32>().unwrap();
        (real, imaginary)
    }
}

fn main() {}
