struct Solution;

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let s = s.as_bytes();
        let sz = s.len();
        let mut line_sweep = vec![0; sz + 1];

        for v in shifts.iter() {
            let (start, end, direction) = (v[0] as usize, v[1] as usize + 1, v[2]);
            if direction == 1 {
                line_sweep[start] += 1;
                line_sweep[end] -= 1;
            } else {
                line_sweep[start] -= 1;
                line_sweep[end] += 1;
            }
        }

        let mut acc: i32 = 0;
        let mut ret = String::new();
        for i in 0..sz {
            acc += line_sweep[i];
            // make shift to be a positive number, acc coulbe be smaller
            let k = (acc % 26 + 26) % 26;
            let target = b'a' as u8 + ((s[i] as i32 - b'a' as i32 + k) % 26) as u8;
            ret.push(target as char);
        }

        ret
    }
}

fn main() {}
