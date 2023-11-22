# Given a 0-indexed integer array nums, determine whether there exist two subarrays of length 2 with equal sum. Note that the two subarrays must begin at different indices.

# Return true if these subarrays exist, and false otherwise.

# A subarray is a contiguous non-empty sequence of elements within an array.

from typing import List

#! Memory Usage > Performance
# class Solution:
#     def findSubarrays(self, nums: List[int]) -> bool:
#         if len(nums) < 2:
#             return False
#         for i in range(len(nums) - 2):
#             for j in range(i + 1, len(nums) - 1):
#                 if nums[i] + nums[i + 1] == nums[j] + nums[j + 1]:
#                     return True
#         return False
    
#! Memory Usage < Performance
class Solution:
    def findSubarrays(self, nums: List[int]) -> bool:
        if len(nums) < 2:
            return False
        values = []
        for i in range(len(nums) - 1):
            if nums[i] + nums[i + 1] in values:
                return True
            values.append(nums[i] + nums[i + 1])
        return False
    
s = Solution()
print(s.findSubarrays([4,2,4]))
print(s.findSubarrays([1,2,3,4,5]))
print(s.findSubarrays([0,0,0]))