#![allow(dead_code)]

use std::collections::HashMap;

struct TweetCounts {
    tweets: HashMap<String, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TweetCounts {
    fn new() -> Self {
        Self {
            tweets: HashMap::new(),
        }
    }

    fn record_tweet(&mut self, tweet_name: String, time: i32) {
        self.tweets.entry(tweet_name).or_default().push(time);
    }

    fn get_tweet_counts_per_frequency(
        &self,
        freq: String,
        tweet_name: String,
        start_time: i32,
        end_time: i32,
    ) -> Vec<i32> {
        let unit;
        match freq.as_ref() {
            "minute" => unit = 60,
            "hour" => unit = 3600,
            "day" => unit = 86400,
            _ => unreachable!(),
        }
        let mut ret = Vec::new();
        for _ in 0..=(end_time - start_time) / unit {
            ret.push(0);
        }
        if let Some(v) = self.tweets.get(&tweet_name) {
            for &time in v.iter() {
                if time >= start_time && time <= end_time {
                    let idx = ((time - start_time) / unit) as usize;
                    ret[idx] += 1;
                }
            }
        }
        ret
    }
}

/**
 * Your TweetCounts object will be instantiated and called as such:
 * let obj = TweetCounts::new();
 * obj.record_tweet(tweetName, time);
 * let ret_2: Vec<i32> = obj.get_tweet_counts_per_frequency(freq, tweetName, startTime, endTime);
 */

fn main() {}
