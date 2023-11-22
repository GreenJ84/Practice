# Given an array arr of positive integers sorted in a strictly increasing order, and an integer k.
# Return the kth positive integer that is missing from this array.

from typing import List


class Solution:
    def findKthPositive(self, arr: List[int], k: int) -> int:
        for i in range(3000):
            if i not in arr:
                k-=1
            if k < 0:
                return i 

s = Solution()
print(s.findKthPositive([2,3,4,7,11],5))
print(s.findKthPositive([1,2,3,4],2))