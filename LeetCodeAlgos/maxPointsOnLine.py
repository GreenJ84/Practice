# Given an array of points where points[i] = [xi, yi] represents a point on the X-Y plane, return the maximum number of points that lie on the same straight line.

import math
from typing import List


class Solution:
    def maxPoints(self, points: List[List[int]]) -> int:
        p = len(points)
        if p == 1:
            return 1
        res = 2
        for i in range(p):
            cnt = {}
            for j in range(p):
                if j != i:
                    slope = math.atan2(
                        points[j][1]-points[i][1],
                        points[j][0]-points[i][0]
                    )
                    if not slope in cnt:
                        cnt[slope]=1
                    else:
                        cnt[slope]+=1
            res = max(res, max(cnt.values())+1)
        return res

s = Solution()
print(s.maxPoints([[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]))
print(s.maxPoints([[1,1]]))