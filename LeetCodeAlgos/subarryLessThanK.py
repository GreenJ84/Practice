# Given an array of integers nums and an integer k, return the number of contiguous subarrays where the product of all the elements in the subarray is strictly less than k.

from typing import List

class Solution(object):
    def numSubarrayProductLessThanK(self, nums, k):
        if k <= 1: return 0
        prod = 1
        ans = left = 0
        for right, val in enumerate(nums):
            prod *= val
            while prod >= k:
                prod /= nums[left]
                left += 1
            ans += right - left + 1
        return ans


# class Solution:
#     def numSubarrayProductLessThanK(self, nums: List[int], k: int) -> int:
#         if min(nums)>=k or k == 0:
#             return 0
#         res = 0
#         if max(nums)<k:
#             res = len(nums)
#         else:
#             for i in nums:
#                 if i < k:
#                     res+=1
        