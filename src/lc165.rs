struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1s: Vec<i32> = version1
            .split(".")
            .map(|c| c.parse::<i32>().unwrap())
            .collect();
        let v2s: Vec<i32> = version2
            .split(".")
            .map(|c| c.parse::<i32>().unwrap())
            .collect();
        for i in 0..v1s.len().max(v2s.len()) {
            let v1 = v1s.get(i).unwrap_or(&0).to_owned();
            let v2 = v2s.get(i).unwrap_or(&0).to_owned();
            if v1 < v2 {
                return -1;
            } else if v1 > v2 {
                return 1;
            }
        }
        return 0;
    }
}

fn main() {
    let s = "1.0.11".to_string();
    for p in s.split(".") {
        println!("{p}");
    }
}
