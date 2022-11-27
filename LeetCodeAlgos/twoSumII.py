# Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.
# Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.
# The tests are generated such that there is exactly one solution. You may not use the same element twice.
# Your solution must use only constant extra space.

from typing import List


class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        x, y = 0, len(numbers)-1
        while x<y:
            if numbers[x]+numbers[y]==target:
                return [x+1, y+1]
            if numbers[x]+numbers[y] > target:
                y-=1
            if numbers[x]+numbers[y] < target:
                x+=1

s = Solution()
print(s.twoSum([2,7,11,15], 9))
print(s.twoSum([2,3,4], 6))
print(s.twoSum([-1,0], -1))