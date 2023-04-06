# Given an integer array nums, return the number of longest increasing subsequences.
# Notice that the sequence has to be strictly increasing.

from typing import List


class Solution:
    def findNumberOfLIS(self, nums: List[int]) -> int:
        n = len(nums)
        dp, cnt = [1]*n, [1]*n
        max_val = 1

        for curr in range(1,n):
            for prev in range(curr):
                if nums[curr] > nums[prev]:
                    if dp[prev]+1 > dp[curr]:
                        dp[curr], cnt[curr] = dp[prev]+1, cnt[prev]
                    elif dp[prev]+1 == dp[curr]:
                        cnt[curr] += cnt[prev]
            max_val = max(max_val, dp[curr])

        return sum([j for i,j in zip(dp,cnt) if i == max_val])

s = Solution()
print(s.findNumberOfLIS([1,3,5,4,7]))
print(s.findNumberOfLIS([2,2,2,2,2]))