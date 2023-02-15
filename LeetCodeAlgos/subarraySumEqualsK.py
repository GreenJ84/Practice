# Given an array of integers nums and an integer k, return the total number of subarrays whose sum equals to k.
# A subarray is a contiguous non-empty sequence of elements within an array.

from functools import reduce
from typing import List


class Solution:
    def subarraySum(self, nums: List[int], k: int) -> int:
        res = 0
        for start in range(len(nums)):
            sum = 0
            for end in range(start, len(nums)):
                sum+=nums[end]
                if sum == k:
                    res+=1
        return res

s = Solution()
print(s.subarraySum([1],0))
print(s.subarraySum([-1,-1,1],0))
print(s.subarraySum([1,1,1],2))
print(s.subarraySum([1,2,3],3))
print(s.subarraySum([1,2,1,2,1],3))
