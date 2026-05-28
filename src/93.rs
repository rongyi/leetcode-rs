struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let s = s.as_bytes();
        let mut out = vec![];
        let mut cur = vec![];
        Self::decode(s, 0, &mut cur, &mut out);

        out
    }

    fn decode(ip: &[u8], pos: usize, cur: &mut Vec<u8>, out: &mut Vec<String>) {
        if pos >= ip.len() {
            if cur.len() == 4 {
                let cur_s = cur
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(".");
                out.push(cur_s);
            }
            return;
        }
        // there's three case
        cur.push(ip[pos] - b'0');
        Self::decode(ip, pos + 1, cur, out);
        cur.pop();

        // can have multiple case
        if ip[pos] != b'0' && pos + 1 < ip.len() {
            let val = (ip[pos] - b'0') * 10 + (ip[pos + 1] - b'0');

            cur.push(val);
            Self::decode(ip, pos + 2, cur, out);
            cur.pop();
        }
        if ip[pos] != b'0' && pos + 2 < ip.len() {
            let val = (ip[pos] - b'0') as usize * 100
                + (ip[pos + 1] - b'0') as usize * 10
                + (ip[pos + 2] - b'0') as usize;
            if val <= 255 {
                cur.push(val as u8);
                Self::decode(ip, pos + 3, cur, out);
                cur.pop();
            }
        }
    }
}

fn main() {}
