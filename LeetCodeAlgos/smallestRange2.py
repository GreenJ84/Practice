# You are given an integer array nums and an integer k.
# For each index i where 0 <= i < nums.length, change nums[i] to be either nums[i] + k or nums[i] - k.
# The score of nums is the difference between the maximum and minimum elements in nums.
# Return the minimum score of nums after changing the values at each index.

from typing import List

class Solution:
    def smallestRangeII(self, nums: List[int], k: int) -> int:
        if len(nums)==1:
            return 0
        nums.sort()
        top, bottom = nums[-1], nums[0]
        ans = top - bottom
        for i in range(len(nums)-1):
            a, b = nums[i], nums[i+1]
            ans = min(ans, max(top-k, a+k) - min(bottom+k, b-k))
        return ans

## 52/69 test cases passed
# class Solution:
#     def smallestRangeII(self, nums: List[int], k: int) -> int:
#         if len(nums)==1:
#             return 0
#         nums.sort()
#         top, bottom = nums[-1], nums[0]
#         for i in range(len(nums)):
#             if top-nums[i] >= nums[i]-bottom:
#                 nums[i] += k
#             else:
#                 nums[i] -= k
#             top, bottom = max(nums), min(nums)
#         return max(nums)-min(nums)

s = Solution()
print(s.smallestRangeII([1], 0))
print(s.smallestRangeII([0, 10], 2))
print(s.smallestRangeII([1, 3, 6], 3))
print(s.smallestRangeII([7, 8, 8], 5))
print(s.smallestRangeII([3, 4, 7, 0], 5))
print(s.smallestRangeII([10, 7, 1], 3))