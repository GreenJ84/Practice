// You are playing the following Nim Game with your friend:

// Initially, there is a heap of stones on the table.
// You and your friend will alternate taking turns, and you go first.
// On each turn, the person whose turn it is will remove 1 to 3 stones from the heap.
// The one who removes the last stone is the winner.
// Given n, the number of stones in the heap, return true if you can win the game assuming both you and your friend play optimally, otherwise return false.

struct Solution {}
impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }

    // pub fn can_win_nim_dp(n: i32) -> bool {
    //     if n <= 3 {
    //         return true;
    //     }
    //     let mut dp = vec![false; (n + 1) as usize];
    //     dp[1] = true;
    //     dp[2] = true;
    //     dp[3] = true;
    //     for i in 4..=n as usize {
    //         dp[i] = !dp[i - 1] || !dp[i - 2] || !dp[i - 3];
    //     }
    //     dp[n as usize]
    // }
}