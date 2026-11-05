struct Solution;

struct TextEditor {
    cursor: usize,
    content: Vec<u8>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TextEditor {
    fn new() -> Self {
        TextEditor {
            cursor: 0,
            content: Vec::with_capacity(10000),
        }
    }

    fn add_text(&mut self, text: String) {
        let b = text.as_bytes();
        let rest: Vec<u8> = self.content[self.cursor..].iter().copied().collect();
        self.content.truncate(self.cursor);
        self.content.extend_from_slice(b);
        self.content.extend_from_slice(&rest);
        // The cursor ends to the right of text
        self.cursor += b.len();
    }

    fn delete_text(&mut self, k: i32) -> i32 {
        let k = k as usize;
        let cut = self.cursor.min(k);
        let head: Vec<u8> = self.content[0..self.cursor - cut].iter().copied().collect();
        let rest: Vec<u8> = self.content[self.cursor..].iter().copied().collect();
        self.content.clear();
        self.content.extend_from_slice(&head);
        self.content.extend_from_slice(&rest);
        self.cursor -= cut;

        cut as _
    }

    fn cursor_left(&mut self, k: i32) -> String {
        let k = k as usize;
        if self.cursor >= k {
            self.cursor -= k;
        } else {
            self.cursor = 0;
        }
        if self.cursor >= 10 {
            let a = self.content[self.cursor - 10..self.cursor].to_vec();
            return String::from_utf8(a).unwrap();
        }

        let a = self.content[..self.cursor].to_vec();

        String::from_utf8(a).unwrap()
    }

    fn cursor_right(&mut self, k: i32) -> String {
        let k = k as usize;
        let mut next_cursor = self.cursor + k;
        if next_cursor > self.content.len() {
            next_cursor = self.content.len();
        }
        self.cursor = next_cursor;
        if self.cursor >= 10 {
            let a = self.content[self.cursor - 10..self.cursor].to_vec();

            return String::from_utf8(a).unwrap();
        }

        let a = self.content[..self.cursor].to_vec();
        String::from_utf8(a).unwrap()
    }
}

fn main() {
    let mut a = vec![1, 2, 3];
    a.insert(3, 10);
    println!("{:?}", a);
}
