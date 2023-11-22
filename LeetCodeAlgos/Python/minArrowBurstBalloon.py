# There are some spherical balloons taped onto a flat wall that represents the XY-plane. The balloons are represented as a 2D integer array points where points[i] = [xstart, xend] denotes a balloon whose horizontal diameter stretches between xstart and xend. You do not know the exact y-coordinates of the balloons.
# Arrows can be shot up directly vertically (in the positive y-direction) from different points along the x-axis. A balloon with xstart and xend is burst by an arrow shot at x if xstart <= x <= xend. There is no limit to the number of arrows that can be shot. A shot arrow keeps traveling up infinitely, bursting any balloons in its path.
# Given the array points, return the minimum number of arrows that must be shot to burst all balloons.

from typing import List, Literal


class Solution:
    def findMinArrowShots(self, points: List[List[int]]) -> int:
        if not points: return 0
        points.sort()
        def check(i: List[int], res:int) -> int:
            no: Literal = True
            for j in wind:
                if i[0]<j[0] and j[1]<i[1]:
                    no = False
                if j[0]<=i[0] and i[0]<=j[1]:
                    no = False
                    j[0]=i[0]
                if j[0]<=i[1] and i[1]<=j[1]:
                    no = False
                    j[1]=i[1]
                if not no: break
            if no:
                res+=1
                wind.append(i)
            return res

        wind: List[List[int]]= [points[0]]
        res: int = 1
        for i in range(1, len(points)):
            res = check(points[i], res)
        return res


s: Solution = Solution()
print(s.findMinArrowShots([[10,16],[2,8],[1,6],[7,12]]))
print(s.findMinArrowShots([[1,2],[3,4],[5,6],[7,8]]))
print(s.findMinArrowShots([[1,2],[2,3],[3,4],[4,5]]))
print(s.findMinArrowShots([[3,9],[7,12],[3,8],[6,8],[9,10],[2,9],[0,9],[3,9],[0,6],[2,8]]))
