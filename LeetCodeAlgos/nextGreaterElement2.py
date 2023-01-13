# Given a circular integer array nums (i.e., the next element of nums[nums.length - 1] is nums[0]), return the next greater number for every element in nums.
# The next greater number of a number x is the first greater number to its traversing-order next in the array, which means you could search circularly to find its next greater number. If it doesn't exist, return -1 for this number.

from typing import List


class Solution:
    def nextGreaterElements(self, nums: List[int]) -> List[int]:
        if len(nums)==1:
            return [-1]
        n = len(nums)
        res = [0]*n
        for i in range(n):
            j = (i+1)%n
            while nums[i]>=nums[j]:
                j=(j+1)%n
                if j == (i+1)%n:
                    res[i]=-1
                    break
            if res[i]!=-1:
                res[i]=nums[j]
        return res

s = Solution()
print(s.nextGreaterElements([1,2,1]))
print(s.nextGreaterElements([1,2,3,4,3]))