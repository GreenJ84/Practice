# Given an array, rotate the array to the right by k steps, where k is non-negative.

from copy import deepcopy
from typing import List


class Solution:
    def rotate(self, nums: List[int], k: int) -> None:
        x = deepcopy(nums)
        for i in range(len(nums)):
            nums[i] = x[(i-k)%len(nums)]
        print(nums)

s = Solution()
s.rotate([1,2,3,4,5,6,7], 3)
s.rotate([-1,-100,3,99], 2)