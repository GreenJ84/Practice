# You are given two integers, x and y, which represent your current location on a Cartesian grid: (x, y). You are also given an array points where each points[i] = [ai, bi] represents that a point exists at (ai, bi). A point is valid if it shares the same x-coordinate or the same y-coordinate as your location.
# Return the index (0-indexed) of the valid point with the smallest Manhattan distance from your current location. If there are multiple, return the valid point with the smallest index. If there are no valid points, return -1.
# The Manhattan distance between two points (x1, y1) and (x2, y2) is abs(x1 - x2) + abs(y1 - y2).


import copy
from typing import List


class Solution:
    def nearestValidPoint(self, x: int, y: int, points: List[List[int]]) -> int:
        p, diff, res = copy.deepcopy(points), 100, []
        for i in p:
            if i[0]!=x and i[1]!=y:
                p.remove(i)
            elif i[0]==x and i[1]==y:
                return 0
            else:
                if abs(i[0]+i[1]-x-y) < diff:
                    diff = i[0]+i[1]-x-y
                    res = [i]
                elif abs(i[0]+i[1]-x-y) == diff:
                    res.append(i)
        if len(res) == 0:
            return -1
        else:
            diff = 100
            for j in p:
                if j[0] < diff:
                    diff = j[0]
        return diff


s = Solution()
print(s.nearestValidPoint(3, 4, [[1,2],[3,1],[2,4],[2,3],[4,4]]))
print(s.nearestValidPoint(3, 4, [[3,4]]))
print(s.nearestValidPoint(3, 4, [[2,3]]))