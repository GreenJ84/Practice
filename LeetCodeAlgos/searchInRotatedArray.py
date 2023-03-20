# There is an integer array nums sorted in ascending order (with distinct values).
# Prior to being passed to your function, nums is possibly rotated at an unknown pivot index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].
# Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.
# You must write an algorithm with O(log n) runtime complexity.

from typing import List

class Solution:
    def search(self, nums: List[int], target: int) -> int:
        low, high = min(nums), max(nums)
        res = [-1]
        def srch(start, end):
            m = start+((end-start)//2)
            if nums[start] == target:
                res.append(start)
                return
            if nums[end] == target:
                res.append(end)
                return
            if end == start or start > end: return
            if nums[m] == target:
                res.append(m)
            elif nums[m] < target:
                if nums[m] < nums[end] and nums[end] < target:
                    srch(start, m-1)
                else:
                    srch(m+1, end)
            else:
                if nums[start] < nums[m] and nums[start] > target:
                    srch(m+1, end)
                else:
                    srch(start, m-1)
        srch(0, len(nums)-1)
        return res[-1]

# class Solution:
#     def search(self, nums: List[int], target: int) -> int:
#         if target not in nums:
#             return -1
#         else:
#             return nums.index(target)

s = Solution()
# print(s.search([4,5,6,7,0,1,2], 0))
# print(s.search([4,5,6,7,0,1,2], 3))
# print(s.search([4,5,6,7,8,1,2,3], 8))
# print(s.search([1], 0))
print(s.search([5,1,2,3,4], 1))