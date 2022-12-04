# You are given an m x n binary matrix grid. An island is a group of 1's (representing land) connected 4-directionally (horizontal or vertical.) You may assume all four edges of the grid are surrounded by water.
# The area of an island is the number of cells with a value 1 in the island.
# Return the maximum area of an island in grid. If there is no island, return 0.

from typing import List


class Solution:
    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        maxArea = 0
        for i in range(len(grid)):
            for j in range(len(grid[0])):
                if grid[i][j] == 1:
                    maxArea = max(islandCheck(1, grid, i, j), maxArea)
        return maxArea

def islandCheck(area, grid, i, j):
    grid[i][j]=0
    if i>0 and grid[i-1][j]==1:
        area = islandCheck(area+1, grid, i-1, j)
    if j<len(grid[0])-1 and grid[i][j+1]==1:
        area = islandCheck(area+1, grid, i, j+1)
    if i<len(grid)-1 and grid[i+1][j]==1:
        area = islandCheck(area+1, grid, i+1, j)
    if j>0 and grid[i][j-1]==1:
        area = islandCheck(area+1, grid, i, j-1)
    return area

s = Solution()

print(s.maxAreaOfIsland([[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]]))
print(s.maxAreaOfIsland([[0,0,0,0,0,0,0,0]]))

