struct Solution;

impl Solution {
    pub fn min_sessions(tasks: Vec<i32>, session_time: i32) -> i32 {
        let sz = tasks.len();
        let done_mask = (1 << sz) - 1;
        // dp[i][j] i mask for all taskon 1 done, 0, not yet
        //          j curr_accumulation
        let mut dp = vec![vec![-1; 16]; 1 << sz];

        Self::help(&tasks, 0, 0, session_time, &mut dp, done_mask)
    }
    fn help(
        tasks: &[i32],
        mut cur_mask: usize,
        mut curr_time: i32,
        session_time: i32,
        dp: &mut Vec<Vec<i32>>,
        done_mask: usize,
    ) -> i32 {
        if curr_time > session_time {
            return tasks.len() as i32;
        }
        if cur_mask == done_mask {
            return 1;
        }
        if dp[cur_mask][curr_time as usize] != -1 {
            return dp[cur_mask][curr_time as usize];
        }
        let mut ret = tasks.len() as i32;

        for i in 0..tasks.len() {
            // not include this task?
            if cur_mask & (1 << i) == 0 {
                let include_in_current = Self::help(
                    tasks,
                    cur_mask | (1 << i),
                    curr_time + tasks[i],
                    session_time,
                    dp,
                    done_mask,
                );
                let include_in_next = 1 + Self::help(
                    tasks,
                    cur_mask | (1 << i),
                    tasks[i],
                    session_time,
                    dp,
                    done_mask,
                );
                ret = ret.min(include_in_current.min(include_in_next));
            }
        }

        dp[cur_mask][curr_time as usize] = ret;
        ret
    }
}

fn main() {}
