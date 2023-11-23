struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let s = s.chars().collect::<Vec<_>>();
        let mut ret: Vec<char> = Vec::new();
        let sz = s.len() as i32;
        let round_sz = num_rows * 2 - 2;
        for i in 0..num_rows {
            let mut round_start = 0;
            while round_start < sz {
                // the up part
                if round_start + i < sz {
                    ret.push(s[(round_start + i) as usize]);
                }
                // down part
                if i != 0 && i != num_rows - 1 && round_start + round_sz - i < sz {
                    ret.push(s[(round_start + round_sz - i) as usize]);
                }

                round_start += round_sz;
            }
        }

        ret.into_iter().collect()
    }
}
