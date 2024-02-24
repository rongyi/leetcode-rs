struct Solution;

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut dir_len: Vec<usize> = Vec::new();

        let mut ret = 0;
        for line in input.split('\n') {
            let mut is_dir = true;
            // file contains a '.'
            if line.contains(".") {
                is_dir = false;
            }
            let line: Vec<char> = line.chars().collect();
            if is_dir {
                let mut i = 0;
                while i < line.len() && line[i] == '\t' {
                    i += 1;
                }
                if i < dir_len.len() {
                    dir_len[i] = line.len() - i;
                } else {
                    dir_len.push(line.len() - i);
                }
            } else {
                let mut cur_len = 0;
                let mut i = 0;
                while i < line.len() && line[i] == '\t' {
                    // and the '/' path
                    cur_len += dir_len[i] + 1;
                    i += 1;
                }
                cur_len += line.len() - i;
                ret = ret.max(cur_len);
            }
        }

        ret as i32
    }
}

fn main() {
    let input = "dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext";
    Solution::length_longest_path(input.to_string());
}
