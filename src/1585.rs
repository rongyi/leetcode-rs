#![allow(dead_code)]

struct Solution;

// Intuition
// If ch[i] > ch[j], we can swap these characters. In other words, we can move a character freely to the left, until it hits a smaller character, e.g.:

// "0231" > "0213" > "0123"

// So, we do not need to sort anything, we just need to check if we can move required characters to the left to get the target string.

// Note: we can also process the string right-to-left and move larger numbers right. In that case, we can just pop used indexes instead tracking them in a separate array pos. I though about it, but it appeared a bit harder to get right during the contest. If interested, check the solution by 0xFFFFFFFF in the comments below.

// Algorithm
// Collect indexes of all characters 0-9 of the source strings in idx. For each characters, we track which indexes we have used in pos.

// For each character ch in the target string, check if we have it in idx. If so, verify that there are no smaller characters in front of it. To do that, we check the current idexes of all characters less than ch.

// If the character can be moved, mark its index as used by advancing pos[ch].

impl Solution {
    pub fn is_transformable(s: String, t: String) -> bool {
        let mut idx: Vec<Vec<usize>> = vec![vec![]; 10];
        let mut pos: Vec<usize> = vec![0; 10];
        for (i, c) in s.chars().enumerate() {
            idx[(c as u8 - b'0') as usize].push(i);
        }
        for c in t.chars() {
            let d = (c as u8 - b'0') as usize;
            if pos[d] >= idx[d].len() {
                return false;
            }

            // 要求是待安排的字符在源字符串里前面不能有比它小的，如果有就安排不了
            // e.g. s: 12345
            //      t: 12435
            // 安排到4的时候没办法了,因为选择大于2的任意包含4的子字符串，3总归是要在4前面的，这个是核心
            // 处理到4的时候遍历0 1 2 3, 发现3在S中的位置是4前面，这就操蛋了，4移不过去
            for i in 0..d {
                if pos[i] < idx[i].len() && idx[i][pos[i]] < idx[d][pos[d]] {
                    return false;
                }
            }
            pos[d] += 1;
        }

        true
    }
}

fn main() {}
