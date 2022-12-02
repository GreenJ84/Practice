# Suppose an array of length n sorted in ascending order is rotated between 1 and n times. For example, the array nums = [0,1,2,4,5,6,7] might become:
    # [4,5,6,7,0,1,2] if it was rotated 4 times.
    # [0,1,2,4,5,6,7] if it was rotated 7 times.
# Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].
# Given the sorted rotated array nums of unique elements, return the minimum element of this array.
#! You must write an algorithm that runs in O(log n) time.

from typing import List


class Solution:
    def findMin(self, nums: List[int]) -> int:
        if len(nums)==1:
            return nums[0]
        start, end = 0, len(nums)-1
        if nums[start] < nums[end]:
            return nums[start]
        while start<=end:
            m = (start+end)//2
            if nums[m] > nums[m+1]:
                return nums[m+1]
            elif nums[m] < nums[m-1]:
                return nums[m]
            elif nums[m] > nums[start]:
                start = m+1
            else:
                end = m-1


s = Solution()
# print(s.findMin([3,4,5,1,2]))
print(s.findMin([5,1,2,3,4]))
# print(s.findMin([4,5,6,7,0,1,2]))
# print(s.findMin([11,13,15,17]))