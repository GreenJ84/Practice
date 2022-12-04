# Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.

from typing import List


class Solution:
    def sortedSquares(self, nums: List[int]) -> List[int]:
        print(sorted(list(map(lambda x: x**2, nums))))

s = Solution()
print(s.sortedSquares([-4, -1, 0, 3, 5]))