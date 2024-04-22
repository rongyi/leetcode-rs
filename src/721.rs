#![allow(dead_code)]


struct Solution;

use std::collections::BTreeSet;
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let emails: Vec<BTreeSet<String>> = accounts
            .iter()
            .map(|v| {
                v.iter()
                    // first is name
                    .skip(1)
                    .map(|e| e.to_string())
                    .collect::<BTreeSet<_>>()
            })
            .collect();

        // construct graph
        let sz = accounts.len();
        let mut graph = vec![vec![]; sz];

        for i in 0..sz {
            for j in i + 1..sz {
                if Self::intersect(&emails[i], &emails[j]) {
                    graph[i].push(j);
                    graph[j].push(i);
                }
            }
        }

        let mut ret = Vec::new();
        let mut visited = vec![false; sz];

        for i in 0..sz {
            if visited[i] {
                continue;
            }
            let mut cur_set = emails[i].clone();
            let mut cur_record = vec![accounts[i][0].clone()];
            Self::dfs(&mut visited, &mut cur_set, &emails, &graph, i);
            for email in cur_set.iter() {
                cur_record.push(email.clone());
            }
            ret.push(cur_record);
        }

        ret
    }

    fn dfs(
        visited: &mut Vec<bool>,
        cur_set: &mut BTreeSet<String>,
        emails: &Vec<BTreeSet<String>>,
        graph: &Vec<Vec<usize>>,
        cur: usize,
    ) {
        visited[cur] = true;
        for &neibs in graph[cur].iter() {
            if visited[neibs] {
                continue;
            }
            // eat them all
            for e in emails[neibs].iter() {
                cur_set.insert(e.clone());
            }
            Self::dfs(visited, cur_set, emails, graph, neibs)
        }
    }

    fn intersect(set1: &BTreeSet<String>, set2: &BTreeSet<String>) -> bool {
        for s in set1.iter() {
            if set2.contains(s) {
                return true;
            }
        }

        false
    }
}

fn main() {}
