# Given a string s, find the longest palindromic subsequence's length in s.
# A subsequence is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements.

class Solution:
    def longestPalindromeSubseq(self, s: str) -> int:
        n = len(s)
        dp = [[0] * n for _ in range(n)]
        for i in range(n):
            dp[i][i] = 1
        
        for k in range(2, n+1):
            for i in range(n-k+1):
                j = i + k - 1
                if s[i] == s[j]:
                    # characters at i and j match, so include them in the palindrome
                    dp[i][j] = 2 + dp[i+1][j-1]
                else:
                    # characters at i and j do not match, so exclude one of them
                    dp[i][j] = max(dp[i+1][j], dp[i][j-1])
        
        return dp[0][n-1]



s = Solution()
print(s.longestPalindromeSubseq("bbbab"))
print(s.longestPalindromeSubseq("cbbd"))
print(s.longestPalindromeSubseq("abcdafhgtalopwaaza"))