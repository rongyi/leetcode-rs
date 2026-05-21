struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut out = vec![];
        let sz = words.len();
        let max_width = max_width as usize;

        let mut cur_line = String::new();
        let mut cur_word_count = 0;
        let mut total_non_space_count = 0;
        let mut i = 0;

        while i < sz {
            let start = i;
            let mut end = i;
            // [start, end)
            // try to fit
            while end < sz && words[end].len() + total_non_space_count + cur_word_count <= max_width
            {
                total_non_space_count += words[end].len();
                cur_word_count += 1;
                end += 1;
            }
            // there must be at least 1 word to fit
            let total_space = max_width - total_non_space_count;
            if cur_word_count == 1 {
                cur_line.push_str(&words[start]);
                cur_line.push_str(&" ".repeat(total_space));
            } else {
                let gap = cur_word_count - 1;
                // last line is special, fuck
                if end == sz {
                    for j in 0..(end - start) {
                        cur_line.push_str(&words[start + j]);
                        if start + j + 1 != end {
                            cur_line.push(' ');
                        }
                    }
                    if cur_line.len() < max_width {
                        cur_line.push_str(&" ".repeat(max_width - cur_line.len()));
                    }
                } else {
                    // even case
                    if total_space % gap == 0 {
                        let each_gap_spaces = total_space / gap;
                        for w in start..end {
                            cur_line.push_str(&words[w]);
                            // except the last
                            if w + 1 != end {
                                cur_line.push_str(&" ".repeat(each_gap_spaces));
                            }
                        }
                    } else {
                        let base_spaces = total_space / gap;
                        let add_from_left = total_space % gap;
                        for j in 0..(end - start) {
                            cur_line.push_str(&words[start + j]);
                            if j < add_from_left {
                                cur_line.push_str(&" ".repeat(base_spaces + 1));
                            } else if start + j + 1 != end {
                                cur_line.push_str(&" ".repeat(base_spaces));
                            }
                        }
                    }
                }
            }

            // next step
            out.push(cur_line);
            cur_line = String::new();
            cur_word_count = 0;
            total_non_space_count = 0;
            i = end;
        }

        out
    }
}

fn main() {
    //["ask   not   what","your country can","do  for  you ask","what  you can do","for your country "]
    //["ask   not   what","your country can","do  for  you ask","what  you can do","for your country"]
}
