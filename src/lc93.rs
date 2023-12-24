
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut cur = Vec::new();
        let mut ret = Vec::new();

        Self::backtrack(&s, 0, 0, &mut cur, &mut ret);

        ret
    }

    fn backtrack(s: &str, pos: usize, parts: i32, cur: &mut Vec<String>, ret: &mut Vec<String>) {
        if pos == s.len() && parts == 4 {
            ret.push(cur.join("."));
            return;
        }
        if parts >= 4 || pos == s.len() {
            return;
        }
        for l in 1..=3 {
            if pos + l > s.len() {
                break;
            }

            let segment = &s[pos..pos + l];
            let num: u32 = segment.parse().unwrap();
            if (segment.starts_with('0') && segment != "0") || num > 255 {
                continue;
            }
            cur.push(segment.to_owned());
            Self::backtrack(s, pos + l, parts + 1, cur, ret);

            cur.pop();
        }
    }
}
