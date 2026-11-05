struct Solution;

#[derive(Default, Debug)]
struct Trie {
    child: [Option<Box<Trie>>; 2],
}

impl Trie {
    fn insert(&mut self, num: i32) {
        let mut node = self;
        for i in (0..32).rev() {
            let bit = (num >> i) & 1;
            node = node.child[bit as usize].get_or_insert_with(|| Box::new(Trie::default()));
        }
    }

    fn find_max_xor(&self, num: i32) -> i32 {
        let mut ret = 0;
        let mut node = self;
        for i in (0..32).rev() {
            let bit = (num >> i) & 1;
            let comlement = 1 - bit;
            if let Some(next) = &node.child[comlement as usize] {
                ret |= 1 << i;
                node = next;
            } else if let Some(next) = &node.child[bit as usize] {
                node = next;
            } else {
                break;
            }
        }

        ret
    }
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut trie = Trie::default();
        for &num in nums.iter() {
            trie.insert(num);
        }

        let mut ret = 0;
        for &num in nums.iter() {
            ret = ret.max(trie.find_max_xor(num));
        }

        ret
    }
}

fn main() {}
