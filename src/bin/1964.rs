struct Solution;

impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut lis = vec![];
        let sz = obstacles.len();
        let mut ret = vec![0; sz];

        for (i, &val) in obstacles.iter().enumerate() {
            if lis.is_empty() || *lis.last().unwrap() <= val {
                lis.push(val);
                ret[i] = lis.len() as i32;
            } else {
                let idx = lis.partition_point(|&v| v <= val);
                // oh we can ensure will not happen in this branch
                lis[idx] = val;
                ret[i] = idx as i32 + 1;
            }
        }

        ret
    }
}

fn main() {}
