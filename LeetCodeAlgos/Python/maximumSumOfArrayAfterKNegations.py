# Given an integer array nums and an integer k, modify the array in the following way:
# choose an index i and replace nums[i] with -nums[i].
# You should apply this process exactly k times. You may choose the same index i multiple times.
# Return the largest possible sum of the array after modifying it in this way.

from typing import List


class Solution:
    def largestSumAfterKNegations(self, nums: List[int], k: int) -> int:
        nums.sort()
        i = 0
        while i < len(nums) and nums[i] < 0:
            nums[i] *= -1
            i += 1
            k -= 1
            if k == 0:
                return sum(nums)
        
        return sum(nums) if k % 2 == 0 else sum(nums) - (min(nums) * 2)
    
s = Solution()
print(s.largestSumAfterKNegations([4,2,3], 1))
print(s.largestSumAfterKNegations([3,-1,0,2], 3))
print(s.largestSumAfterKNegations([2,-3,-1,5,-4], 2))