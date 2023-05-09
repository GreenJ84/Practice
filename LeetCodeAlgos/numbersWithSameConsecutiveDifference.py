# Given two integers n and k, return an array of all the integers of length n where the difference between every two consecutive digits is k. You may return the answer in any order.
# Note that the integers should not have leading zeros. Integers as 02 and 043 are not allowed.

from typing import List


class Solution:
    def numsSameConsecDiff(self, n: int, k: int) -> List[int]:
        self.ans = []
        for i in range(1, 10):
            self.buildValidSet(n-1, k, i, i)
        return self.ans

    def buildValidSet(self, n, k, prev, res):
        # Add full new valid set
        if n == 0:
            self.ans.append(res)
            return
        
        res = res * 10
        for i in range(0, 10):
            if abs(prev - i) == k:
                self.buildValidSet(n - 1, k, i, res + i)


s = Solution()
print(s.numsSameConsecDiff(3, 7)
        == [181,292,707,818,929]
    )
print(s.numsSameConsecDiff(2, 1)
        == [10,12,21,23,32,34,43,45,54,56,65,67,76,78,87,89,98]
    )