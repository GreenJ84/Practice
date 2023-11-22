# You are given an array coordinates, coordinates[i] = [x, y], where [x, y] represents the coordinate of a point. Check if these points make a straight line in the XY plane.

from typing import List


class Solution:
    def checkStraightLine(self, coordinates: List[List[int]]) -> bool:
        if len(coordinates) == 2:
            return True

        (x0, y0), (x1, y1) = coordinates[0], coordinates[1]

        for i in range(2, len(coordinates)):
            x, y = coordinates[i]
            if (x0-x1)*(y1-y) != (x1-x)*(y0-y1):
                return False
        return True


s = Solution()
print(s.checkStraightLine([[1,2],[2,3],[3,4],[4,5],[5,6],[6,7]]))
print(s.checkStraightLine([[1,1],[2,2],[3,4],[4,5],[5,6],[7,7]]))