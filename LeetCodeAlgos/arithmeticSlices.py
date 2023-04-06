# An integer array is called arithmetic if it consists of at least three elements and if the difference between any two consecutive elements is the same.
# For example, [1,3,5,7,9], [7,7,7,7], and [3,-1,-5,-9] are arithmetic sequences.
# Given an integer array nums, return the number of arithmetic subarrays of nums.
# A subarray is a contiguous subsequence of the array.

from typing import List


class Solution:
    def numberOfArithmeticSlices(self, nums: List[int]) -> int:
        n = len(nums)
        ans = 0
        dp = [0]*n

        if len(nums) < 3: return ans

        for curr in range(2, n):
            if nums[curr] - nums[curr-1] == nums[curr-1] - nums[curr-2]:
                dp[curr] = dp[curr-1]+1
                ans += dp[curr]

        return ans
    
s = Solution()
print(s.numberOfArithmeticSlices([1,2,3,4]))
print(s.numberOfArithmeticSlices([1]))
