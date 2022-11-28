# Given an integer array nums, handle multiple queries of the following type:
# Calculate the sum of the elements of nums between indices left and right inclusive where left <= right.
# Implement the NumArray class:
    # NumArray(int[] nums) Initializes the object with the integer array nums.
    # int sumRange(int left, int right) Returns the sum of the elements of nums between indices left and right inclusive (i.e. nums[left] + nums[left + 1] + ... + nums[right]).

from typing import List


class NumArray:

    def __init__(self, nums: List[int]):
        self.list = nums
        for i in range(len(self.list)-1):
            self.list[i+1]+=self.list[i]

    def sumRange(self, left: int, right: int) -> int:
        if left == 0:
            return self.list[right]
        else:
            return self.list[right]-self.list[left-1]
