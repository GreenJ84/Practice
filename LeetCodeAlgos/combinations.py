# Given two integers n and k, return all possible combinations of k numbers chosen from the range [1, n].
# You may return the answer in any order.


from typing import List

# from itertools import combinations
# class Solution:
#     def combine(self, n: int, k: int) -> List[List[int]]:
#         return [list(k) for k in combinations(range(1, n + 1), k)]


class Solution:
    def combine(self, n: int, k: int) -> List[List[int]]:
        if n == k:
            return [[i for i in range(1,n+1)]]
        def build(x):
            if len(x)==k:
                s = x
                s.sort()
                if s not in ans:
                    ans.append(s)
            else:
                for i in range(n):
                    if i+1 not in x:
                        build(x[:]+[i+1])

        ans = []
        build([])
        return ans

s = Solution()
print(s.combine(13, 13))
print(s.combine(3, 3))
# print(s.combine(4, 2))
# print(s.combine(1, 1))