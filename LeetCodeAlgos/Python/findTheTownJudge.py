# In a town, there are n people labeled from 1 to n. There is a rumor that one of these people is secretly the town judge.
# If the town judge exists, then:
    # The town judge trusts nobody.
    # Everybody (except for the town judge) trusts the town judge.
    # There is exactly one person that satisfies properties 1 and 2.
# You are given an array trust where trust[i] = [ai, bi] representing that the person labeled ai trusts the person labeled bi. If a trust relationship does not exist in trust array, then such a trust relationship does not exist.
# Return the label of the town judge if the town judge exists and can be identified, or return -1 otherwise.

import functools
from typing import List


class Solution:
    def findJudge(self, n: int, trust: List[List[int]]) -> int:
        if n == 1: return 1
        if len(trust) < n-1: return -1
        t = {}
        g = {}
        for i in trust:
            if i[1] in t:
                t[i[1]] += 1
            else:
                t[i[1]] = 1
            if i[0] in g:
                g[i[0]] += 1
            else:
                g[i[0]] = 1
        judge = functools.reduce(lambda a, b: a if a[1] > b[1] else b, list(t.items()))
        return judge[0] if not judge[0] in g and judge[1] == n-1 else -1

s = Solution()
# print(s.findJudge(2, [[1,2]]))
print(s.findJudge(2, [[1,2],[2,1]]))
# print(s.findJudge(3, [[1,3],[2,3]]))
# print(s.findJudge(3, [[1,3],[2,3],[3,1]]))
# print(s.findJudge(3, [[1,2],[2,3]]))