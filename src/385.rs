#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

/// LeetCode 385: Mini Parser
///
/// Deserialize a string like "[123,[456,[789]]]" into a NestedInteger.
/// Stack-based parsing: '[' pushes a new list, digits accumulate,
/// ',' and ']' finalize the current number.
impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        // Plain integer without brackets.
        if !s.contains('[') {
            return NestedInteger::Int(s.parse().unwrap());
        }

        let bytes = s.as_bytes();
        let mut stack: Vec<Vec<NestedInteger>> = Vec::new();
        let mut num = 0i32;
        let mut neg = false;
        let mut has_num = false; // whether we're in the middle of parsing a number

        for &b in bytes {
            match b {
                b'[' => {
                    stack.push(Vec::new());
                    has_num = false;
                }
                b']' | b',' => {
                    // Finalize any pending number.
                    if has_num {
                        let val = if neg { -num } else { num };
                        stack.last_mut().unwrap().push(NestedInteger::Int(val));
                        num = 0;
                        neg = false;
                        has_num = false;
                    }
                    // ',' just separates — the number is already added.
                    // ']' closes the current list and attaches it to its parent.
                    if b == b']' {
                        let finished = NestedInteger::List(stack.pop().unwrap());
                        if stack.is_empty() {
                            return finished;
                        }
                        stack.last_mut().unwrap().push(finished);
                    }
                }
                b'-' => {
                    neg = true;
                    has_num = true;
                }
                d if d.is_ascii_digit() => {
                    num = num * 10 + (d - b'0') as i32;
                    has_num = true;
                }
                _ => {} // whitespace (shouldn't appear, but be safe)
            }
        }

        unreachable!()
    }
}

fn main() {}
