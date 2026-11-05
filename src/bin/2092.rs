struct Solution;

struct UnionFind {
    group: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            group: (0..n).collect(),
        }
    }
    fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
    fn connect(&mut self, a: usize, b: usize) {
        let pida = self.find(a);
        let pidb = self.find(b);
        // chain together
        self.group[pida] = pidb;
    }
    fn find(&mut self, id: usize) -> usize {
        if self.group[id] == id {
            return id;
        }
        let pid = self.find(self.group[id]);
        self.group[id] = pid;
        pid
    }
    fn reset(&mut self, id: usize) {
        self.group[id] = id;
    }
}

impl Solution {
    pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        // by time
        meetings.sort_by_key(|x| x[2]);
        let mut uf = UnionFind::new(n as usize);
        uf.connect(0, first_person as usize);
        let mut ppl: Vec<i32> = Vec::new();
        let mut sz = meetings.len();
        let mut i = 0;
        while i < sz {
            ppl.clear();
            let cur_time = meetings[i][2];
            while i < sz && meetings[i][2] == cur_time {
                uf.connect(meetings[i][0] as usize, meetings[i][1] as usize);
                ppl.push(meetings[i][0]);
                ppl.push(meetings[i][1]);

                i += 1;
            }
            for &p in ppl.iter() {
                if !uf.connected(0, p as usize) {
                    uf.reset(p as usize);
                }
            }
        }

        (0..n).filter(|&x| uf.connected(0, x as usize)).collect()
    }
}

fn main() {}
