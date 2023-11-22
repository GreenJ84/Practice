# Given an integer array nums, return the length of the longest strictly increasing 
# subsequence
# .

from typing import List


class Solution:
    def lengthOfLIS(self, nums: List[int]) -> int:
        n = len(nums)
        dp = [1]*n
        # Iterate through all i items
        for num in range(n):
            # Iterater all j previous to j
            for prev in range(num):
                if nums[num] > nums[prev]:
                    dp[num]=max(dp[num], dp[prev]+1)

        return max(dp)
    
s = Solution()
print(s.lengthOfLIS([10,9,2,5,3,7,101,18]))
print(s.lengthOfLIS([0,1,0,3,2,3]))
print(s.lengthOfLIS([7,7,7,7,7,7,7]))
print(s.lengthOfLIS([4,10,4,3,8,9]))