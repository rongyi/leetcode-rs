struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut output = vec![];
        let mut cur_word_cnt = 0;
        let mut non_space_char_cnt = 0;

        let mut i = 0;
        let sz = words.len();
        let mut chunk_start = 0;
        while i < sz {
            // try fit test
            // ok, add this line
            if cur_word_cnt + non_space_char_cnt + words[i].len() <= max_width {
                // simply add to this line
                non_space_char_cnt += words[i].len();
                cur_word_cnt += 1;
            } else {
                let mut cur_line = String::new();
                if cur_word_cnt > 1 {
                    let space = (max_width - non_space_char_cnt) / (cur_word_cnt - 1);
                    let extr_space = (max_width - non_space_char_cnt) % (cur_word_cnt - 1);

                    for j in chunk_start..i {
                        cur_line.push_str(&words[j]);
                        if j + 1 < i {
                            cur_line.push_str(&" ".repeat(space));
                            // extraspace
                            if j - chunk_start < extr_space {
                                cur_line.push(' ');
                            }
                        }
                    }
                } else {
                    cur_line.push_str(&words[chunk_start]);
                    cur_line.push_str(&" ".repeat(max_width - non_space_char_cnt));
                }
                output.push(cur_line);

                // not zero
                cur_word_cnt = 1;
                non_space_char_cnt = words[i].len();
                chunk_start = i;
            }

            i += 1;
        }

        let mut cur_line = String::new();
        // For the last line of text, it should be left-justified, and no extra space is inserted between words.
        for i in chunk_start..sz {
            cur_line.push_str(&words[i]);
            cur_line.push(' ');
        }
        if cur_line.len() < max_width {
            let last_part = max_width - cur_line.len();
            cur_line.push_str(&" ".repeat(last_part));
        }
        // incase the ' ' for every word
        cur_line.truncate(max_width);
        output.push(cur_line);

        output
    }
}
fn main() {}
