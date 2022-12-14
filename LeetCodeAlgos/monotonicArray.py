# An array is monotonic if it is either monotone increasing or monotone decreasing.
# An array nums is monotone increasing if for all i <= j, nums[i] <= nums[j]. An array nums is monotone decreasing if for all i <= j, nums[i] >= nums[j].
# Given an integer array nums, return true if the given array is monotonic, or false otherwise.

from typing import List

#* 91% runtime/85% Meory
class Solution:
    def isMonotonic(self, nums: List[int]) -> bool:
        if nums[0]<nums[len(nums)-1]:
            inc = True
        else:
            inc = False

        for i in range(0, len(nums)-1):
            if inc:
                if not nums[i]<=nums[i+1]:
                    return False
            else:
                if not nums[i]>=nums[i+1]:
                    return False
        return True

#* Attempt 1: Passe with bad runtime
# class Solution:
#     def isMonotonic(self, nums: List[int]) -> bool:
#         x = self.check(nums)
#         nums.reverse()
#         return (x or self.check(nums))
        
#     def check(self, nums):
#         inc, dec, i = True, True, 0
#         for j in range(len(nums)-1, 0, -1):
#             if nums[i] > nums[i+1]:
#                 inc = False
#             if nums[j] < nums[j-1]:
#                 dec = False
#             i+=1
#         return (inc or dec)

s = Solution()
# print(s.isMonotonic([1,2,2,3]))
# print(s.isMonotonic([6,5,4,4]))
# print(s.isMonotonic([1,3,2]))
print(s.isMonotonic([3,1,9]))