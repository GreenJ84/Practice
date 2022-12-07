# Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
# Note that you must do this in-place without making a copy of the array.

from typing import List


class Solution:
    def moveZeroes(self, nums: List[int]) -> None:
        i = pop = 0
        while i<len(nums)-pop:
            if nums[i] == 0:
                nums.pop(i)
                nums.append(0)
                pop+=1
                i -= 1
            i+=1

s = Solution()

x = [0,1,0,3,12]
y = [0]
z = [0,0,1]

# s.moveZeroes(x)
# s.moveZeroes(y)
s.moveZeroes(z)

# print(x)
# print(y)
print(z)