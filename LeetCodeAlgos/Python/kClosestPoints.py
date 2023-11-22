# Given an array of points where points[i] = [xi, yi] represents a point on the X-Y plane and an integer k, return the k closest points to the origin (0, 0).
# The distance between two points on the X-Y plane is the Euclidean distance (i.e., √(x1 - x2)2 + (y1 - y2)2).
# You may return the answer in any order. The answer is guaranteed to be unique (except for the order that it is in).

import math
from typing import List

# Better Runtime & Memory. Passes all tests
class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        res = [[p[0]**2+p[1]**2, p] for p in points]
        res = sorted(res, key=lambda x: x[0])
        return [res[i][1] for i in range(k)]

#! # Runtime constraints. Times out at high input volume
# def distance(_x2: int, _y2: int)-> int:
#     return _x2**2+_y2**2
# class Solution:
#     def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
#         n = len(points)
#         res = []
#         dist = []
#         for i in range(n):
#             dist.append(distance(points[i][0], points[i][1]))
#         for j in range(k):
#             mi = dist.index(min(dist))
#             res.append(points[mi])
#             dist[mi] = math.inf
#         return res

s = Solution()
print(s.kClosest([[1,3],[-2,2]], 1))
print(s.kClosest([[3,3],[5,-1],[-2,4]], 2))
