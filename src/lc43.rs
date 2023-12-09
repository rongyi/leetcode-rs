struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let sz1 = num1.len();
        let sz2 = num2.len();
        let mut ret: Vec<u32> = vec![0; sz1 + sz2];

        let num1: Vec<u32> = num1
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let num2: Vec<u32> = num2
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        for i in 0..sz1 {
            for j in 0..sz2 {
                let mul = num1[i] * num2[j];
                let sum = ret[i + j] + mul;

                ret[i + j] = sum % 10;
                ret[i + j + 1] += sum / 10;
            }
        }

        while ret.len() > 1 && ret.last() == Some(&0) {
            ret.pop();
        }

        ret.into_iter()
            .rev()
            .map(|d| std::char::from_digit(d, 10).unwrap())
            .collect()
    }
}
