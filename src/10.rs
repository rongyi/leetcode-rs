struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Self::re_match(&s.as_bytes(), &p.as_bytes())
    }

    fn re_match(s: &[u8], p: &[u8]) -> bool {
        if p.is_empty() {
            return s.is_empty();
        }
        match p[0] {
            b'.' => {
                if s.is_empty() {
                    if p.len() > 1 && p[1] == b'*' {
                        return Self::re_match(s, &p[2..]);
                    }
                    return false;
                }
                if p.len() == 1 || p[1] != b'*' {
                    // increment one step for reach
                    return Self::re_match(&s[1..], &p[1..]);
                }

                // let first = s[0];
                // one ore more
                for (i, &_o) in s.iter().enumerate() {
                    // if o != first {
                    //     break;
                    // }
                    if Self::re_match(&s[i + 1..], &p[2..]) {
                        return true;
                    }
                }

                // zero case
                Self::re_match(s, &p[2..])
            }
            c @ _ => {
                // peek for *
                // ok, no * is possible
                if p.len() == 1 || p[1] != b'*' {
                    if s.is_empty() || s[0] != c {
                        return false;
                    }
                    return Self::re_match(&s[1..], &p[1..]);
                }
                // now the process of c*
                // 1. no match, just ignore patten
                if Self::re_match(s, &p[2..]) {
                    return true;
                }

                // 2. one or more, for each
                for (i, &o) in s.iter().enumerate() {
                    if o == c {
                        if Self::re_match(&s[i + 1..], &p[2..]) {
                            return true;
                        }
                    } else {
                        break;
                    }
                }
                false
            }
        }
    }
}

fn main() {}
