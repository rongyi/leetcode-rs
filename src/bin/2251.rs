struct Solution;

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut starts = vec![];
        let mut ends = vec![];
        for f in flowers.iter() {
            starts.push(f[0]);
            ends.push(f[1]);
        }
        starts.sort_unstable();
        ends.sort_unstable();
        let mut ret = vec![];

        for p in people.iter() {
            let started = starts.partition_point(|x| *x < *p + 1);
            let ended = ends.partition_point(|x| *x < *p);
            ret.push((started - ended) as i32);
        }

        ret
    }
}

fn main() {}
