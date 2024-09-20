struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
        let sz = graph.len();
        let mut parent = vec![-1; sz];
        let mut cnt = HashMap::new();

        // at least each group containt itself
        for i in 0..sz {
            cnt.insert(i, 1);
        }
        // then union find to join group together
        for i in 0..sz {
            for j in i + 1..sz {
                if graph[i][j] == 1 {
                    let parx = Self::find(&mut parent, i);
                    let pary = Self::find(&mut parent, j);
                    if parx != pary {
                        Self::mkunion(&mut parent, parx, pary);
                        let &numy = cnt.get(&pary).unwrap();
                        // merge ygroup to xgroup
                        cnt.entry(parx).and_modify(|x| {
                            *x += numy;
                        });
                    }
                }
            }
        }
        let mut ret = *initial.iter().min().unwrap();
        let mut seen = HashMap::new();
        for &init in initial.iter() {
            // i.e. its group
            let group = Self::find(&mut parent, init as usize);
            *seen.entry(group).or_insert(0) += 1;
        }
        let mut maxi = 0;
        for &init in initial.iter() {
            let par = Self::find(&mut parent, init as usize);
            if seen[&par] == 1 {
                if cnt[&par] > maxi {
                    maxi = cnt[&par];
                    ret = init;
                } else if cnt[&par] == maxi {
                    ret = ret.min(init);
                }
            }
        }

        ret as i32
    }
    fn find(parent: &mut Vec<i32>, x: usize) -> usize {
        if parent[x as usize] == -1 {
            return x;
        }
        Self::find(parent, parent[x] as usize)
    }
    fn mkunion(parent: &mut Vec<i32>, x: usize, y: usize) {
        parent[y] = x as i32;
        parent[x] = -1;
    }
}

fn main() {}
