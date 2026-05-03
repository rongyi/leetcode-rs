struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let table: Vec<Vec<u8>> = [
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ]
        .iter()
        .map(|s| s.as_bytes().iter().copied().collect::<Vec<u8>>())
        .collect();
        let mut ret = vec![];
        let mut cur = vec![];

        Self::recur(digits.as_bytes(), 0, &table, &mut ret, &mut cur);

        ret
    }

    fn recur(
        digits: &[u8],
        idx: usize,
        table: &Vec<Vec<u8>>,
        output: &mut Vec<String>,
        cur: &mut Vec<u8>,
    ) {
        if idx == digits.len() {
            let cur_s = String::from_utf8(cur.clone()).unwrap();
            output.push(cur_s);
            return;
        }
        let table_pos = (digits[idx] - b'0') as usize;

        for &c in table[table_pos].iter() {
            cur.push(c);
            Self::recur(digits, idx + 1, table, output, cur);
            cur.pop();
        }
    }
}

fn main() {}
