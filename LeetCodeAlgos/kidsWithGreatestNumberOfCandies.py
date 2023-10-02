# There are n kids with candies. You are given an integer array candies, where each candies[i] represents the number of candies the ith kid has, and an integer extraCandies, denoting the number of extra candies that you have.

# Return a boolean array result of length n, where result[i] is True if, after giving the ith kid all the extraCandies, they will have the greatest number of candies among all the kids, or False otherwise.

# Note that multiple kids can have the greatest number of candies.




from typing import List


class Solution:
    def kidsWithCandies(self, candies: List[int], extraCandies: int) -> List[bool]:
        max_candy_holding = max(candies)
        res = []
        for holdings in candies:
            if holdings + extraCandies >= max_candy_holding:
                res.append(True)
            else:
                res.append(False)
        return res


s = Solution()
test1 = s.kidsWithCandies([2,3,5,1,3], 3)
print(test1)
assert test1 == [True,True,True,False,True]

test2 = s.kidsWithCandies([4,2,1,1,2], 1)
print(test2)
assert test2 == [True,False,False,False,False]

test3 = s.kidsWithCandies([12,1,12], 10)
print(test3)
assert test3 == [True,False,True]