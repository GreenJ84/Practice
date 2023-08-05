# You are given an n x n grid where we place some 1 x 1 x 1 cubes that are axis-aligned with the x, y, and z axes.

# Each value v = grid[i][j] represents a tower of v cubes placed on top of the cell (i, j).

# We view the projection of these cubes onto the xy, yz, and zx planes.

# A projection is like a shadow, that maps our 3-dimensional figure to a 2-dimensional plane. We are viewing the "shadow" when looking at the cubes from the top, the front, and the side.

# Return the total area of all three projections.

from itertools import count
from typing import List

class Solution:
    def projectionArea(self, grid: List[List[int]]) -> int:
        # Get x, y area
        area = len(grid) * len(grid[0]) - sum(list(map(lambda x: x.count(0), grid)))
        # Add x, z area
        area += sum(list(map(lambda x: max(x), grid)))
        # Add y, z area
        area += sum(list(map(lambda y: max(y), zip(*grid))))
        return area


s = Solution()
print(s.projectionArea([[1,2],[3,4]]))
print(s.projectionArea([[2]]))
print(s.projectionArea([[1,0],[0,2]]))