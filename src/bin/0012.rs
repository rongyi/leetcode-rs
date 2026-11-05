struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        // [0, 3999]
        let mut r = String::new();
        let q = num / 1000;
        if q > 0 {
            let cur = "M".repeat(q as usize);
            r.push_str(&cur);
        }
        num %= 1000;
        let q = num / 100;
        if q > 0 {
            let cur = if q == 4 {
                "CD".to_string()
            } else if q == 9 {
                "CM".to_string()
            } else if q == 5 {
                "D".to_string()
            } else if q < 5 {
                "C".repeat(q as usize)
            } else {
                let mut f = "D".to_string();
                let f2 = "C".repeat((q - 5) as usize);
                f.push_str(&f2);
                f
            };

            r.push_str(&cur);
        }

        num %= 100;
        let q = num / 10;
        if q > 0 {
            let cur = if q == 4 {
                "XL".to_string()
            } else if q == 9 {
                "XC".to_string()
            } else if q == 5 {
                "L".to_string()
            } else if q < 5 {
                "X".repeat(q as usize)
            } else {
                let mut f = "L".to_string();
                let f2 = "X".repeat((q - 5) as usize);
                f.push_str(&f2);
                f
            };
            r.push_str(&cur);
        }
        num %= 10;
        let q = num;
        if q > 0 {
            let cur = if q == 4 {
                "IV".to_string()
            } else if q == 9 {
                "IX".to_string()
            } else if q == 5 {
                "V".to_string()
            } else if q < 5 {
                "I".repeat(q as usize)
            } else {
                let mut f = "V".to_string();
                let f2 = "I".repeat((q - 5) as usize);
                f.push_str(&f2);
                f
            };
            r.push_str(&cur);
        }

        r
    }
}

fn main() {}
