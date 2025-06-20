struct Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let n = n as usize;
        let mut neibs = vec![vec![]; n + 1];

        for e in edges.iter() {
            let n1 = e[0] as usize;
            let n2 = e[1] as usize;
            neibs[n1].push(n2);
            neibs[n2].push(n1);
        }
        let mut min_steps = vec![10001; n + 1];
        let mut steps = 0;
        let mut q = VecDeque::new();
        q.push_back(1);

        while !q.is_empty() && steps <= min_steps[n] + 1 {
            let cursz = q.len();
            for i in 0..cursz {
                let cur = q.pop_front().unwrap();

                if steps == min_steps[cur] || steps > min_steps[cur] + 1 {
                    continue;
                }
                min_steps[cur] = min_steps[cur].min(steps);

                if cur == n && steps > min_steps[n] {
                    return Self::stepsToTime(steps, time, change);
                }
                for item in neibs[cur].iter() {
                    q.push_back(*item);
                }
            }

            steps += 1;
        }

        Self::stepsToTime(min_steps[n] + 2, time, change)
    }

    fn stepsToTime(steps: i32, time: i32, change: i32) -> i32 {
        let mut ret = 0;

        for _ in 0..steps - 1 {
            ret += time;
            if (ret / change) % 2 != 0 {
                ret = (ret / change + 1) * change;
            }
        }

        ret + time
    }
}

impl Solution {
    pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
        let sz = parents.len();
        let mut child = vec![vec![]; sz];
        let mut cnt = vec![1i64; sz];

        for i in 1..sz {
            child[parents[i] as usize].push(i);
        }
        Self::calculate(&mut cnt, 0, &child);

        let mut max_count = 1;
        let mut max_score = 1;

        for &c in child[0].iter() {
            max_score *= cnt[c];
        }

        // delete each node
        for i in 1..sz {
            let parent_score = cnt[0] - cnt[i];
            let mut cur_score = parent_score;

            for &c in child[i].iter() {
                cur_score *= cnt[c];
            }
            if cur_score > max_score {
                max_score = cur_score;
                max_count = 1;
            } else if cur_score == max_score {
                max_count += 1
            }
        }

        max_count
    }

    fn calculate(cnt: &mut Vec<i64>, cur: usize, child: &Vec<Vec<usize>>) -> i64 {
        for &c in child[cur].iter() {
            cnt[cur as usize] += Self::calculate(cnt, c, child);
        }

        cnt[cur]
    }
}

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

