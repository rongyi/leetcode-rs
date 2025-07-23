pub struct Solution;

impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        let mut total_time = vec![0; n];
        let mut indegree = vec![0; n];
        for r in relations.iter() {
            graph[(r[0] - 1) as usize].push((r[1] - 1) as usize);
            indegree[(r[1] - 1) as usize] += 1;
        }
        let mut q = std::collections::VecDeque::new();
        for i in 0..n {
            if indegree[i] == 0 {
                q.push_back(i);
                // no prev constrain, their time is just the learning time
                total_time[i] = time[i];
            }
        }
        while let Some(cur) = q.pop_front() {
            for &next_course in graph[cur].iter() {
                total_time[next_course] =
                    total_time[next_course].max(total_time[cur] + time[next_course]);
                indegree[next_course] -= 1;

                if indegree[next_course] == 0 {
                    q.push_back(next_course);
                }
            }
        }
        *total_time.iter().max().unwrap()
    }
}

fn main() {}
