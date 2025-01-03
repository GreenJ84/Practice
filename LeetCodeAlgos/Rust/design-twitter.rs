/**
 * Design a simplified version of Twitter where users can post tweets, follow/unfollow another user, and is able to see the 10 most recent tweets in the user's news feed.
 *
 * Implement the Twitter class:
 *
 * Twitter() Initializes your twitter object.
 * void postTweet(int userId, int tweetId) Composes a new tweet with ID tweetId by the user userId. Each call to this function will be made with a unique tweetId.
 * List<Integer> getNewsFeed(int userId) Retrieves the 10 most recent tweet IDs in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user themself. Tweets must be ordered from most recent to least recent.
 * void follow(int followerId, int followeeId) The user with ID followerId started following the user with ID followeeId.
 * void unfollow(int followerId, int followeeId) The user with ID followerId started unfollowing the user with ID followeeId.
 */

use std::collections::*;

struct Twitter {
    // UserId -> Set(followingIds)
    following: HashMap<i32, HashSet<i32>>,
    // (userId, tweetId)
    tweets: Vec<(i32, i32)>
}
impl Twitter {
    fn new() -> Self {
        Twitter {
            following: HashMap::new(),
            tweets: Vec::new()
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.push((user_id, tweet_id));
    }

    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let following: HashSet<i32> = self.following
            .get(&user_id)
            .map_or_else(|| HashSet::from([user_id]), |set| {
                let mut set = set.clone();
                set.insert(user_id);
                set
            });

        let mut feed: Vec<i32> = self.tweets
            .iter()
            .filter(|(uid, _)| following.contains(uid))
            .map(|(_, tid)| *tid)
            .collect();
        feed.reverse();
        feed.truncate(10);
        feed
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id == followee_id { return; }
        self.following.entry(follower_id)
            .or_insert_with(HashSet::new)
            .insert(followee_id);
        // if let Some(followers) = self.following.get(follower_id) {
        //     followers.insert(followee_id)
        // } else {
        //     self.following.insert(follower_id, HashSet::from([followee_id]));
        // }
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(followers) = self.following.get_mut(&follower_id) {
            followers.remove(&followee_id);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let mut twitter: Twitter = Twitter::new();
        twitter.post_tweet(1, 5);
        assert_eq!(twitter.get_news_feed(1), Vec::from([5]));
        twitter.follow(1, 2);
        twitter.post_tweet(2, 6);
        assert_eq!(twitter.get_news_feed(1), Vec::from([6, 5]));
        twitter.unfollow(1, 2);
        assert_eq!(twitter.get_news_feed(1), Vec::from([5]));
    }
}