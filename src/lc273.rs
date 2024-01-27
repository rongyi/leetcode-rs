struct Solution;

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        let units = ["", "Thousand", "Million", "Billion"];
        let mut num = num;
        let mut result = String::new();

        for unit in &units {
            let part = Self::helper(num % 1000).trim().to_owned();
            if !part.is_empty() {
                result = format!("{} {} {}", part, unit, result);
            }
            num /= 1000;
        }

        result.trim().to_string()
    }

    fn helper(num: i32) -> String {
        let below_20 = [
            "",
            "One",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six",
            "Seven",
            "Eight",
            "Nine",
            "Ten",
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen",
        ];

        let tens = [
            "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ];

        if num == 0 {
            "".to_string()
        } else if num < 20 {
            below_20[num as usize].to_string()
        } else if num < 100 {
            format!("{} {}", tens[num as usize / 10], Self::helper(num % 10))
        } else {
            format!(
                "{} Hundred {}",
                below_20[num as usize / 100],
                Self::helper(num % 100)
            )
        }
    }
}

fn main() {}
