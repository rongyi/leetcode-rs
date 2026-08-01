struct Solution;

#[derive(Debug, Clone, Copy)]
struct Process {
    pid: usize,
    is_start: bool,
    timestamp: i32,
}

impl Process {
    fn new(pid: usize, is_start: bool, timestamp: i32) -> Self {
        Self {
            pid,
            is_start,
            timestamp,
        }
    }
}

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut ret = vec![0; n as usize];
        // (pid, is_start, timestamp)
        let mut stack: Vec<usize> = Vec::new();

        let mut prev_task_start_time = 0;
        for log in &logs {
            let cur = Self::parse(log);
            if cur.is_start {
                if let Some(&top_id) = stack.last() {
                    ret[top_id] += cur.timestamp - prev_task_start_time;
                }
                stack.push(cur.pid);
                prev_task_start_time = cur.timestamp;
            } else {
                // this is much better understanding
                let top_id = stack.pop().unwrap();
                ret[top_id] += cur.timestamp - prev_task_start_time + 1;
                // start next
                prev_task_start_time = cur.timestamp + 1;
            }
        }
        ret
    }

    fn parse(s: &str) -> Process {
        let chunks: Vec<&str> = s.split(':').collect();
        let pid = chunks[0].parse::<usize>().ok().unwrap();
        let is_start = chunks[1] == "start";
        let ts = chunks[2].parse::<i32>().ok().unwrap();
        Process::new(pid, is_start, ts)
    }
}

fn main() {}
