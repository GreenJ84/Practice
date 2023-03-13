# Given an integer array nums, find a  subarray that has the largest product, and return the product.
# The test cases are generated so that the answer will fit in a 32-bit integer.

from typing import List

class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        # hold max and min values
        # min * a negative will be a max
        mn, mx = 1, 1
        # start with the max single value
        ans = max(nums)
        # iterate nums
        for num in nums:
            # product of zero is always zero / reset
            if num == 0:
                mx, mn = 1, 1
                continue
            # hold min value
            temp = mn * num
            # find new min
            mn = min(mn*num, mx*num, num)
            # find new max
            mx = max(temp, mx*num, num)
            # is max larger? 
            ans = max(ans, mx)
        return ans

# class Solution:
#     def maxProduct(self, nums: List[int]) -> int:
#         ans = -float('inf')
#         for start in range(0, len(nums)):
#             mx = nums[start]
#             ans = max(ans, mx)
#             for end in range(start+1, len(nums)):
#                 mx *= nums[end]
#                 ans = max(ans, mx)
#         return ans

s = Solution()
print(s.maxProduct([2,3,-2,4]))
print(s.maxProduct([-2,0,-1]))
print(s.maxProduct([-2]))
print(s.maxProduct([-3, 0, 1, -2]))
print(s.maxProduct([2,-5,-2,-4,3]))




