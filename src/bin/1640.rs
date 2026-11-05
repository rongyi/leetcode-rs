#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        use std::collections::HashMap;

        let mut map = HashMap::new();

        // Map the first number of each piece to its piece
        for piece in pieces.iter() {
            if !piece.is_empty() {
                map.insert(piece[0], piece);
            }
        }

        let mut i = 0;
        while i < arr.len() {
            if let Some(piece) = map.get(&arr[i]) {
                // Check if the piece matches at the current position
                if i + piece.len() > arr.len() {
                    return false;
                }

                for j in 0..piece.len() {
                    if arr[i + j] != piece[j] {
                        return false;
                    }
                }

                // Move past this piece
                i += piece.len();
            } else {
                // If we can't find a piece starting with the current number
                return false;
            }
        }

        true
    }
}
fn main() {}
