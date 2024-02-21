use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Tweet {
    id: i32,
    time: i32,
}

#[derive(Debug)]
struct Twitter {
    user_tweets: HashMap<i32, Vec<Tweet>>,
    follows: HashMap<i32, HashSet<i32>>,
    time: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Twitter {
            user_tweets: HashMap::new(),
            follows: HashMap::new(),
            time: 0,
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.user_tweets
            .entry(user_id)
            .or_insert(Vec::new())
            .push(Tweet {
                id: tweet_id,
                time: self.time,
            });
        self.time += 1;
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let b = vec![];
        // user himself tweets
        let mut tweets: Vec<&Tweet> = self
            .user_tweets
            .get(&user_id)
            .unwrap_or(&b)
            .iter()
            .collect();
        for &followee_id in self.follows.get(&user_id).unwrap_or(&HashSet::new()) {
            if let Some(user_tweets) = self.user_tweets.get(&followee_id) {
                tweets.extend(user_tweets.iter());
            }
        }
        tweets.sort_by_key(|tweet| -tweet.time);
        tweets.iter().map(|tweet| tweet.id).take(10).collect()
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows
            .entry(follower_id)
            .or_insert_with(HashSet::new)
            .insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(set) = self.follows.get_mut(&follower_id) {
            set.remove(&followee_id);
        }
    }
}

fn main() {}
