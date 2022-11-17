# Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
# You may assume that each input would have exactly one solution, and you may not use the same element twice.
# You can return the answer in any order.

from typing import List

import copy
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        x = copy.deepcopy(nums)
        x.sort()

        f, b = 0, len(nums)-1
        while(f<b):
            if x[f]+x[b] == target:
                if x[f] != x[b]:
                    return [nums.index(x[f]), nums.index(x[b])]
                else:
                    return equalVals(x[f], x[b], nums)
            elif x[f]+x[b] > target:
                b -= 1
            else:
                f += 1
        if x[f]==x[b] and x[f]+x[b] == target:
            return equalVals(x[f], x[b], nums)
        return [0]

def equalVals(left, right, nums):
    a = nums.index(left)
    nums.pop(a)
    b = nums.index(right)+1
    return [a, b]

s = Solution()
print(s.twoSum([2,7,11,15], 9))
print(s.twoSum([3,2,4], 6))
print(s.twoSum([3,3], 6))