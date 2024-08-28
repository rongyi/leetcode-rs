struct Solution;
use std::collections::HashMap;


impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let sz = quiet.len();
        let mut rich_chain: HashMap<i32, Vec<i32>> = HashMap::new();

        for p in richer.iter() {
            rich_chain.entry(p[1]).or_default().push(p[0]);
        }

        let mut ret = vec![-1; sz];
        for i in 0..sz {
            Self::dfs(&rich_chain, i, &mut ret, &quiet);
        }

        ret
    }
    fn dfs(
        rich_chain: &HashMap<i32, Vec<i32>>,
        cur: usize,
        ret: &mut Vec<i32>,
        quite: &Vec<i32>,
    ) -> i32 {
        if ret[cur] >= 0 {
            return ret[cur];
        }
        ret[cur] = cur as i32;
        if let Some(neibs) = rich_chain.get(&(cur as i32)) {
            for &rich in neibs.iter() {
                if quite[ret[cur] as usize]
                    > quite[Self::dfs(rich_chain, rich as usize, ret, quite) as usize]
                {
                    ret[cur] = ret[rich as usize];
                }
            }
        }

        ret[cur]
    }
}

fn main() {}
