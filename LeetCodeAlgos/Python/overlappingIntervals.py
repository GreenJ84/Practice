# Given an array of intervals intervals where intervals[i] = [starti, endi], return the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping.

from typing import List


class Solution:
    def eraseOverlapIntervals(self, intervals: List[List[int]]) -> int:
        intervals.sort(key=lambda x: x[0])
        pre = -float("inf")
        res = 0

        for i in intervals:
            if i[0] < pre:
                res+=1
                if i[1] < pre:
                    pre = i[1]
            else:
                pre = i[1]
        return res

s = Solution()
print(s.eraseOverlapIntervals([[1,2],[2,3],[3,4],[1,3]]))
print(s.eraseOverlapIntervals([[1,2],[1,2],[1,2]]))
print(s.eraseOverlapIntervals([[1,2],[2,3]]))
print(s.eraseOverlapIntervals([[1,100],[11,22],[1,11],[2,12]]))