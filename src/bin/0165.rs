struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1s: Vec<_> = version1.split('.').collect();
        let v2s: Vec<_> = version2.split('.').collect();
        let sz1 = v1s.len();
        let sz2 = v2s.len();
        let max_sz = sz1.max(sz2);
        for i in 0..max_sz {
            let v1: i32 = if let Some(s) = v1s.get(i) {
                s.parse().unwrap()
            } else {
                0
            };
            let v2: i32 = if let Some(s) = v2s.get(i) {
                s.parse().unwrap()
            } else {
                0
            };
            if v1 < v2 {
                return -1;
            } else if v1 > v2 {
                return 1;
            }
        }

        0
    }
}

fn main() {}
