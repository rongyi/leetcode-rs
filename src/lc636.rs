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
        let mut stack: Vec<Process> = Vec::new();

        for log in &logs {
            let cur = Self::parse(log);
            // is start?
            if cur.is_start {
                stack.push(cur);
            } else {
                let duration = cur.timestamp - (*stack.last().unwrap()).timestamp + 1;
                ret[cur.pid] += duration;
                stack.pop();
                if !stack.is_empty() {
                    ret[(*stack.last().unwrap()).pid] -= duration;
                }
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
