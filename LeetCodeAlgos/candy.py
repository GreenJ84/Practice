# There are n children standing in a line. Each child is assigned a rating value given in the integer array ratings.
# You are giving candies to these children subjected to the following requirements:
# Each child must have at least one candy.
# Children with a higher rating get more candies than their neighbors.
# Return the minimum number of candies you need to have to distribute the candies to the children.

import math
from typing import List


class Solution:
    def candy(self, ratings: List[int]) -> int:
        res = [1]*len(ratings)
        for i in range(1, len(ratings)):
            if ratings[i]>ratings[i-1]:
                res[i] += res[i-1]

        for i in range(len(ratings)-2, -1,-1):
            if ratings[i]>ratings[i+1]:
                res[i] = max(res[i], res[i+1]+1)

        return sum(res)

s = Solution()
print(s.candy([1,0,2]))
print(s.candy([1,2,2]))
print(s.candy([1,3,2,2,1]))
print(s.candy([1,2,87,87,87,2,1]))