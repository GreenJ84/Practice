# Given an integer array nums, return true if there exists a triple of indices (i, j, k) such that i < j < k and nums[i] < nums[j] < nums[k]. If no such indices exists, return false.

from math import inf
from typing import List

class Solution:
    def increasingTriplet(self, nums: List[int]) -> bool:
        fst = inf
        sec = inf
        for num in nums:
            if num <= fst:
                fst = num
            elif num <= sec: 
                sec = num
            else:
                return True
        return False

# class Solution:
#     def increasingTriplet(self, nums: List[int]) -> bool:
#         n = len(nums)
#         sub = [nums[0]]
#         for i in range(1,n):
#             if len(sub) > 0 and nums[i] <= sub[0]:
#                 sub[0] = nums[i]
#             elif len(sub) == 0: 
#                 sub.append(nums[i])
#             elif len(sub) > 1 and nums[i] <= sub[1]:
#                 sub[1] = nums[i]
#             elif len(sub) == 1 and nums[i] > sub[0]:
#                 sub.append(nums[i])
#             else:
#                 return True
#         return False
    
s = Solution()
print(s.increasingTriplet([1,2,3,4,5]))
print(s.increasingTriplet([5,4,3,2,1]))
print(s.increasingTriplet([2,1,5,0,4,6]))
print(s.increasingTriplet([1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]))
