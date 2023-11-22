# You are given an array of non-overlapping intervals intervals where intervals[i] = [starti, endi] represent the start and the end of the ith interval and intervals is sorted in ascending order by starti. You are also given an interval newInterval = [start, end] that represents the start and end of another interval.
# Insert newInterval into intervals such that intervals is still sorted in ascending order by starti and intervals still does not have any overlapping intervals (merge overlapping intervals if necessary).
# Return intervals after the insertion.

from typing import List


class Solution:
    def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
        # empty intervals
        if len(intervals) == 0:
            intervals.append(newInterval)
            return intervals
        # single interval
        if len(intervals) == 1:
            if newInterval[0] > intervals[0][1]:
                return intervals+[newInterval]
            elif newInterval[1] < intervals[0][0]:
                return [newInterval]+intervals
            else:
                return [[min(newInterval[0], intervals[0][0]), max(newInterval[1], intervals[0][1])]]


        idx = 0
        while idx < len(intervals)-1:
            # If newInterval start is between interval starts
            if newInterval[0] >= intervals[idx][0] and newInterval[0] < intervals[idx+1][0]:
                # no overlap
                if newInterval[0] > intervals[idx][1] and newInterval[1] < intervals[idx+1][0]:
                    return intervals[:idx+1]+[newInterval]+intervals[idx+1:]
                # If newInterval overlaps the proceeding interval
                elif newInterval[0] <= intervals[idx][1]:
                    # The interval will be intergrated with the preceding
                    hold = (idx, intervals[idx][0])
                else:
                    # the interval will start by itself
                    hold = (idx+1, newInterval[0])
                # Loop to find where it will end
                while idx < len(intervals)-1:
                    if newInterval[1] < intervals[idx+1][0]:
                        overlapFix = [hold[1], max(newInterval[1], intervals[idx][1])]
                        return intervals[:hold[0]]+[overlapFix]+intervals[idx+1:]
                    idx += 1
                overlapFix = [hold[1], max(newInterval[1], intervals[idx][1])]
                return intervals[:hold[0]]+[overlapFix]


            # If the newInterval starts before the first interval
            elif idx == 0 and newInterval[0] < intervals[idx][0]:
                # there is no overlap in the newInterval
                if newInterval[1] < intervals[idx][0]:
                    return [newInterval]+intervals
                # the newInterval only overlaps the first interval
                elif newInterval[1] <= intervals[idx][1]: 
                    intervals[idx] = [newInterval[0], max(newInterval[1], intervals[idx][1])]
                    return intervals
                # See how far the overlap is beyond first interval
                else:
                    intervals[0][0] = newInterval[0]
                    while idx < len(intervals)-1:
                        if newInterval[1] < intervals[idx+1][0]:
                            intervals[0][1] = max(intervals[idx][1], newInterval[1])
                            return intervals
                        idx += 1
                    intervals[0][1] = max(intervals[-1][1], newInterval[1])
                    return intervals[:1]


            # if the newInterval starts after the last interval
            elif idx == len(intervals)-2 and newInterval[0] >= intervals[idx+1][0]:
                # there is no overlap
                if newInterval[0] > intervals[idx+1][1]:
                    return intervals+[newInterval]
                # dictate overlap betwen newInterval and last interval
                else:
                    intervals[idx+1] = [intervals[idx+1][0], max(intervals[idx+1][1], newInterval[1])]
                    return intervals
            idx+=1
s = Solution()
print(s.insert([[1,3],[6,9]], [2,5]))
print(s.insert([[1,2],[3,5],[6,7],[8,10],[12,16]], [4,8]))
print(s.insert([[1,2],[3,5],[6,7],[8,9],[12,16]], [10,11]))


ne = [1,2,3,4,7,8,9]
print(ne[:4]+[5,6]+ne[5:])