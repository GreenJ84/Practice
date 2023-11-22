# You have a long flowerbed in which some of the plots are planted, and some are not. However, flowers cannot be planted in adjacent plots.
# Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, and an integer n, return if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule.

from typing import List


class Solution:
    def canPlaceFlowers(self, flowerbed: List[int], n: int) -> bool:
        for s in range(len(flowerbed)):
            if flowerbed[s] == 0:
                if (s-1 < 0 or flowerbed[s-1] == 0) and (s+1 >= len(flowerbed) or flowerbed[s+1] == 0):
                    flowerbed[s] = 1 
                    n -= 1
        return n <= 0

s = Solution()
print(s.canPlaceFlowers([1,0,0,0,1], 1))
print(s.canPlaceFlowers([1,0,0,0,1], 2))
print(s.canPlaceFlowers([0,0,1,0,1], 1))