struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut ret = i32::MAX;
        let mut times: Vec<i32> = time_points
            .iter()
            .map(|time| {
                let parts: Vec<&str> = time.split(':').collect();
                let hours = parts[0].parse::<i32>().unwrap_or(0);
                let minutes = parts[1].parse::<i32>().unwrap_or(0);
                hours * 60 + minutes
            })
            .collect();
        times.sort();
        for i in 1..times.len() {
            ret = ret.min(times[i] - times[i - 1]);
        }
        let val = *times.last().unwrap() - *times.first().unwrap();
        let val = val.min(24 * 60 - val);
        ret = ret.min(val);

        ret
    }
}

fn main() {
    let s = "00".parse::<i32>().unwrap();
    println!("{}", s);
}
