# Given an array of integers arr, replace each element with its rank.

# The rank represents how large the element is. The rank has the following rules:

# Rank is an integer starting from 1.
# The larger the element, the larger the rank. If two elements are equal, their rank must be the same.
# Rank should be as small as possible.

from typing import List


class Solution:
    def arrayRankTransform(self, arr: List[int]) -> List[int]:
        ranks = list( sorted( set(arr[:]) ) )
        return list ( map ( lambda x: ranks.index(x)+1, arr ) )
    
s = Solution()
print(s.arrayRankTransform([40,10,20,30]))
print(s.arrayRankTransform([100,100,100]))
print(s.arrayRankTransform([37,12,28,9,100,56,80,5,12]))