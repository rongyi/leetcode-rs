#![allow(dead_code)]

struct Solution;

// leetcode 1410
impl Solution {
    pub fn entity_parser(text: String) -> String {
        // List of entities sorted by length descending
        let entities = [
            ("&frasl;".as_bytes(), b'/'),
            ("&quot;".as_bytes(), b'"'),
            ("&apos;".as_bytes(), b'\''),
            ("&amp;".as_bytes(), b'&'),
            ("&gt;".as_bytes(), b'>'),
            ("&lt;".as_bytes(), b'<'),
        ];

        let text_bytes = text.as_bytes();
        let mut result = Vec::with_capacity(text.len());
        let mut i = 0;

        while i < text_bytes.len() {
            if text_bytes[i] == b'&' {
                let mut matched = false;
                // Check each entity in order of descending length
                for (entity, replacement) in &entities {
                    let entity_len = entity.len();
                    if i + entity_len <= text_bytes.len() {
                        if &text_bytes[i..i + entity_len] == *entity {
                            result.push(*replacement);
                            i += entity_len;
                            matched = true;
                            break;
                        }
                    }
                }
                if !matched {
                    result.push(b'&');
                    i += 1;
                }
            } else {
                result.push(text_bytes[i]);
                i += 1;
            }
        }

        String::from_utf8(result).unwrap()
    }
}

fn main() {}
