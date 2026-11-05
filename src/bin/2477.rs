struct Solution;

impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let sz = roads.len() + 1;
        let mut graph: Vec<Vec<usize>> = vec![vec![]; sz];

        for r in roads.iter() {
            let (from, to) = (r[0] as usize, r[1] as usize);
            graph[from].push(to);
            graph[to].push(from);
        }
        // include itself child count, aka subtree number
        let mut children: Vec<i64> = vec![0; sz];
        for &neib in graph[0].iter() {
            Self::dfs(neib, 0, &mut children, &graph);
        }
        let mut fuel = 0;
        let seats = seats as i64;

        for i in 1..sz {
            // wtf is this?
            // childres[i] means this subtree(including root) number
            // so for all nodes except node 0, we count how many cars we need
            // for current subtree, those cars will cost 1 * cars fuels to uppper node
            // and then for every upper node, we count how many cars we need(including
            // the subtree car) and to move upper layer, also cost (1 * cars) fuels
            // 0----1-----3
            // |    \
            // 5     4
            //        \
            //         2
            // seats = 3
            // childrens[2] = 1 -> one car
            // ...
            // childres[1] = 4 -> need two car, so from node 1 to capital 0 this step
            // need 2 fuels
            fuel += (children[i] + seats - 1) / seats
        }

        fuel
    }

    fn dfs(cur: usize, parent: usize, children: &mut Vec<i64>, graph: &Vec<Vec<usize>>) -> i64 {
        let mut ret = 1;

        for &next in graph[cur].iter() {
            if next == parent {
                continue;
            }
            ret += Self::dfs(next, cur, children, graph);
        }

        children[cur] = ret;

        ret
    }
}

fn main() {}
