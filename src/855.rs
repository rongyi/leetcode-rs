use std::collections::BTreeSet;

#[derive(Debug)]
struct ExamRoom {
    n: i32,
    seated: BTreeSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ExamRoom {
    fn new(n: i32) -> Self {
        Self {
            n,
            seated: BTreeSet::new(),
        }
    }

    fn seat(&mut self) -> i32 {
        let mut prev = -1;
        let mut max_dist = 0;
        let mut ret = -1;

        if self.seated.is_empty() {
            self.seated.insert(0);
            return 0;
        }

        for &pos in self.seated.iter() {
            if prev == -1 {
                // first pos is empty, now new student sit there
                if pos != 0 {
                    ret = 0;
                    max_dist = pos;
                }
            } else {
                let cur_dist = (pos - prev) / 2;
                if cur_dist > max_dist {
                    max_dist = cur_dist;
                    ret = prev + cur_dist;
                }
            }

            prev = pos;
        }
        // sit at the last pos
        if self.n - 1 - prev > max_dist {
            ret = self.n - 1;
        }
        self.seated.insert(ret);

        ret
    }

    fn leave(&mut self, p: i32) {
        self.seated.remove(&p);
    }
}

/**
 * Your ExamRoom object will be instantiated and called as such:
 * let obj = ExamRoom::new(n);
 * let ret_1: i32 = obj.seat();
 * obj.leave(p);
 */

fn main() {}
