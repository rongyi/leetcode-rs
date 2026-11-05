struct Solution;

impl Solution {
    // when you are given this problem in interview, just go!
    pub fn is_valid(code: String) -> bool {
        let code: Vec<char> = code.chars().collect();
        let mut i = 0;

        let ok = Self::valid_tag(&code, &mut i);
        ok && i == code.len()
    }

    fn valid_tag(s: &[char], i: &mut usize) -> bool {
        let mut j = *i;
        let tag = Self::parse_tag_name(s, &mut j);
        if tag.is_empty() {
            return false;
        }
        if !Self::valid_content(s, &mut j) {
            return false;
        }
        let k = j + tag.len() + 2;
        let expect = "</".to_string() + &tag + ">";
        if k >= s.len() || s[j..=k].iter().collect::<String>() != expect {
            return false;
        }
        *i = k + 1;
        true
    }

    fn parse_tag_name(s: &[char], i: &mut usize) -> String {
        if s[*i] != '<' {
            return "".to_string();
        }
        if let Some((j, _)) = s.iter().enumerate().skip(*i).find(|(_, &x)| x == '>') {
            if j - *i - 1 < 1 || j - *i - 1 > 9 {
                return "".to_string();
            }
            let ret: String = s[*i + 1..j].iter().collect();
            for c in ret.chars() {
                if !c.is_uppercase() {
                    return "".to_string();
                }
            }
            *i = j + 1;
            ret
        } else {
            "".to_string()
        }
    }
    fn valid_text(s: &[char], i: &mut usize) -> bool {
        let j = *i;
        while *i < s.len() && s[*i] != '<' {
            *i += 1;
        }
        *i != j
    }

    fn valid_cdata(s: &[char], i: &mut usize) -> bool {
        let cdata: Vec<char> = "<![CDATA[".chars().collect();

        if !s[*i..].starts_with(&cdata) {
            return false;
        }
        let chunk: String = s.iter().skip(*i + 1).collect();
        let idx = chunk.find("]]>");
        if let Some(idx) = idx {
            let idx = idx + *i + 1;
            *i = idx + 3;
            true
        } else {
            false
        }
    }
    fn valid_content(s: &[char], i: &mut usize) -> bool {
        let mut j = *i;
        while j < s.len() {
            if !Self::valid_text(s, &mut j)
                && !Self::valid_cdata(s, &mut j)
                && !Self::valid_tag(s, &mut j)
            {
                break;
            }
            *i = j;
        }
        true
    }
}

fn main() {
    let s = "hello]]>".to_string();
    let ss: String = s.chars().skip(2 + 1).collect();
    let idx = ss.find("]]>");
    println!("{:?}", idx.unwrap() + 3);
    let ret = Solution::is_valid("<DIV>This is the first line <![CDATA[<div>]]></DIV>".to_string());
    println!("{:?}", ret);
}
