struct Solution;

impl Solution {
    pub fn largest_time_from_digits(arr: Vec<i32>) -> String {
        let mut arr = arr;
        arr.sort_unstable();
        let mut max_time = -1;
        let mut ret = String::new();

        Self::permute(&mut arr, 0, &mut |perm| {
            let hours = perm[0] * 10 + perm[1];
            let minutes = perm[2] * 10 + perm[3];

            if hours < 24 && minutes < 60 {
                let time = hours * 60 + minutes;
                if time > max_time {
                    max_time = time;
                    ret = format!("{:02}:{:02}", hours, minutes);
                }
            }
        });

        ret
    }

    fn permute<F>(arr: &mut [i32], start: usize, f: &mut F)
    where
        F: FnMut(&[i32]),
    {
        if start == arr.len() {
            f(arr);
            return;
        }
        for i in start..arr.len() {
            arr.swap(start, i);
            Self::permute(arr, start + 1, f);
            arr.swap(start, i);
        }
    }
}

fn main() {}
