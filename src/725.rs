struct Solution;

impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let sz = Self::len(head.as_ref());
        let k = k as usize;

        let avg = sz / k;
        let left = sz % k;
        let mut alloc = vec![avg; k];
        for i in 0..left {
            alloc[i] += 1;
        }

        let mut ret = vec![None; k];
        let mut p = head;

        for i in 0..k {
            let expect_sz = alloc[i];
            if expect_sz == 0 {
                continue;
            }
            let mut dummy = ListNode::new(0);
            let mut tail = &mut dummy;
            for _ in 0..expect_sz {
                if let Some(mut node) = p {
                    p = node.next.take();
                    tail.next = Some(node);
                    tail = tail.next.as_mut().unwrap();
                }
            }
            ret[i] = dummy.next;
        }

        ret
    }

    fn len(mut head: Option<&Box<ListNode>>) -> usize {
        let mut sz = 0;
        while let Some(node) = head {
            sz += 1;
            head = node.next.as_ref();
        }
        sz
    }
}

fn main() {}
