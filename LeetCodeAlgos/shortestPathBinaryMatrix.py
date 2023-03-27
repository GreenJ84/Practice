# Given an n x n binary matrix grid, return the length of the shortest clear path in the matrix. If there is no clear path, return -1.
# A clear path in a binary matrix is a path from the top-left cell (i.e., (0, 0)) to the bottom-right cell (i.e., (n - 1, n - 1)) such that:
# All the visited cells of the path are 0.
# All the adjacent cells of the path are 8-directionally connected (i.e., they are different and they share an edge or a corner).
# The length of a clear path is the number of visited cells of this path.

from typing import List

class Solution:
    def shortestPathBinaryMatrix(self, grid: List[List[int]]) -> int:
        n = len(grid)
        # No starting/end point
        if grid[0][0] == 1 or grid[n-1][n-1] == 1:
            return -1
        minPath = [float('inf')]
        seen = []

        def dps(x, y, path, seen):
            # stop current search if grid is not open
            if [x, y] in seen:
                return
            seen.append([x,y])
            # if is, increase current path
            path += 1
            # if at end check path
            if x == 0 and y == 0:
                minPath[0] = min(minPath[0], path)
                return
            # If shorter path already stop curr search
            if minPath[0] <= path:
                return
            tX, tY = x-1, y-1
            bX, bY = x+1, y+1
            # top
            if tX >= 0 and grid[tX][y] == 0:
                dps(tX, y, path, seen[:])
            # top left
            if tX >= 0 and tY >= 0 and grid[tX][tY] == 0:
                dps(tX, tY, path, seen[:])
            # left
            if tY >= 0 and grid[x][tY] == 0:
                dps(x, tY, path, seen[:])
            # bottom left
            if bX < n and tY >= 0 and grid[bX][tY] == 0:
                dps(bX, tY, path, seen[:])
            # bottom
            if bX < n and grid[bX][y] == 0:
                dps(bX, y, path, seen[:])
            # bottom right
            if bX < n and bY < n and grid[bX][bY] == 0:
                dps(bX, bY, path, seen[:])
            # right
            if bY < n and grid[x][bY] == 0:
                dps(x, bY, path, seen[:])
            # top right
            if tX >= 0 and bY < n and grid[tX][bY] == 0:
                dps(tX, bY, path, seen[:])
        dps(n-1, n-1, 0, seen)

        return minPath[0] if minPath[0] != float('inf') else -1

s = Solution()
# print(s.shortestPathBinaryMatrix([[0,1],[1,0]]))
print(s.shortestPathBinaryMatrix([[0,0,0],[1,1,0],[1,1,0]]))
print(s.shortestPathBinaryMatrix([[1,0,0],[1,1,0],[1,1,0]]))
print(s.shortestPathBinaryMatrix([[0,1,1,0,0,0],[0,1,0,1,1,0],[0,1,1,0,1,0],[0,0,0,1,1,0],[1,1,1,1,1,0],[1,1,1,1,1,0]]))
print(s.shortestPathBinaryMatrix([[0,1,0,0,0,0],[0,1,0,1,1,0],[0,1,1,0,1,0],[0,0,0,0,1,0],[1,1,1,1,1,0],[1,1,1,1,1,0]]))
print(s.shortestPathBinaryMatrix([[0,0,0,0,0],[1,1,0,1,0],[0,1,1,1,1],[1,1,1,1,0],[0,1,1,0,0]]))