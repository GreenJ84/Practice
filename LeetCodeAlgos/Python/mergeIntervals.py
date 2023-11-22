# Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.

from typing import List

# Horrible Runtime
class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        intervals.sort(key=lambda x: x[0])
        i = 1
        while i < len(intervals):
            if intervals[i][0] <= intervals[i-1][1]:
                if intervals[i][1] > intervals[i-1][1]:
                    intervals[i-1][1] = intervals[i][1]
                intervals = intervals[:i]+intervals[i+1:]
                i-=1
            i+=1
        return intervals


s = Solution()
print(s.merge([[1,3],[2,6],[8,10],[15,18]]))
print(s.merge([[1,4],[4,5]]))
print(s.merge([[1,4],[2,3]]))
print(s.merge([[1,4],[0,2],[3,5]]))