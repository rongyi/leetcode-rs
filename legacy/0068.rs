struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut ret = Vec::new();
        let mut cur_line: Vec<String> = Vec::new();
        let mut cur_words_sz = 0;

        let max_width: usize = max_width as usize;
        for w in words.iter() {
            // words len + spaces + current word len
            let peek = cur_words_sz + cur_line.len() + w.len();
            if peek <= max_width {
                cur_line.push(w.clone());
                cur_words_sz += w.len();
            } else {
                if cur_line.len() == 1 {
                    let mut cur = cur_line[0].clone();
                    cur.push_str(&" ".repeat(max_width - cur.len()));
                    ret.push(cur);
                } else {
                    // average spaces
                    let div = (max_width - cur_words_sz) / (cur_line.len() - 1);
                    // padding from left
                    let rest = (max_width - cur_words_sz) % (cur_line.len() - 1);
                    let mut cur = cur_line[0].clone();
                    for i in 1..cur_line.len() {
                        if i <= rest {
                            cur.push_str(&" ".repeat(div + 1));
                        } else {
                            cur.push_str(&" ".repeat(div));
                        }
                        cur.push_str(&cur_line[i]);
                    }
                    ret.push(cur);
                }

                cur_line.clear();
                cur_line.push(w.clone());
                cur_words_sz = w.len();
            }
        }

        let mut cur = cur_line[0].clone();
        for i in 1..cur_line.len() {
            cur.push(' ');
            cur.push_str(&cur_line[i]);
        }
        cur.push_str(&" ".repeat(max_width - cur.len()));
        ret.push(cur);

        ret
    }
}
