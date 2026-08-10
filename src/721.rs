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

            cur_record.extend(cur_set.into_iter());
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

mod another {

    struct Solution;
    use std::collections::{BTreeSet, HashMap};

    struct UnionFind {
        parent: Vec<usize>,
    }

    impl UnionFind {
        fn new(n: usize) -> Self {
            Self {
                parent: (0..n).collect(),
            }
        }

        fn find(&mut self, i: usize) -> usize {
            if self.parent[i] != i {
                self.parent[i] = self.find(self.parent[i]);
            }
            self.parent[i]
        }

        fn union(&mut self, i: usize, j: usize) {
            let root_i = self.find(i);
            let root_j = self.find(j);
            if root_i != root_j {
                self.parent[root_i] = root_j;
            }
        }
    }

    impl Solution {
        pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
            let n = accounts.len();
            let mut uf = UnionFind::new(n);
            let mut email_to_acc: HashMap<&str, usize> = HashMap::new();

            // Step 1: Union accounts that share emails
            for (i, acc) in accounts.iter().enumerate() {
                for email in acc.iter().skip(1) {
                    if let Some(&prev_acc) = email_to_acc.get(email.as_str()) {
                        uf.union(i, prev_acc);
                    } else {
                        email_to_acc.insert(email.as_str(), i);
                    }
                }
            }

            // Step 2: Group emails by representative root account index
            let mut merged: HashMap<usize, BTreeSet<String>> = HashMap::new();
            for (email, &acc_idx) in &email_to_acc {
                let root = uf.find(acc_idx);
                merged.entry(root).or_default().insert(email.to_string());
            }

            // Step 3: Construct output vector [Name, email1, email2, ...]
            let mut result = Vec::with_capacity(merged.len());
            for (root, emails) in merged {
                let mut account = Vec::with_capacity(emails.len() + 1);
                account.push(accounts[root][0].clone());
                account.extend(emails);
                result.push(account);
            }

            result
        }
    }
}

fn main() {}
