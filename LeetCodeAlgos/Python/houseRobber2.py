# You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed. All houses at this place are arranged in a circle. That means the first house is the neighbor of the last one. Meanwhile, adjacent houses have a security system connected, and it will automatically contact the police if two adjacent houses were broken into on the same night.
# Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.

from typing import List


class Solution:
    def rob(self, nums: List[int]) -> int:
        return max(nums[0] + robbing(nums[2:-1]), robbing(nums[1:]))

def robbing( nums):
    dp1, dp2 = 0, 0
    for num in nums:
        dp1, dp2 = dp2, max(dp1 + num, dp2)
    return dp2


s = Solution()
print(s.rob([2,7,9,3,1]))
print(s.rob([1,2,3,1]))
# print(s.rob([2,3,2]))
# print(s.rob([1,2,3]))
print(s.rob([2,1,1,2]))
print(s.rob([200,3,140,20,10]))