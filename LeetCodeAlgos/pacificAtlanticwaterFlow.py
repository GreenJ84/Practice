# There is an m x n rectangular island that borders both the Pacific Ocean and Atlantic Ocean. The Pacific Ocean touches the island's left and top edges, and the Atlantic Ocean touches the island's right and bottom edges.
# The island is partitioned into a grid of square cells. You are given an m x n integer matrix heights where heights[r][c] represents the height above sea level of the cell at coordinate (r, c).
# The island receives a lot of rain, and the rain water can flow to neighboring cells directly north, south, east, and west if the neighboring cell's height is less than or equal to the current cell's height. Water can flow from any cell adjacent to an ocean into the ocean.
# Return a 2D list of grid coordinates result where result[i] = [ri, ci] denotes that rain water can flow from cell (ri, ci) to both the Pacific and Atlantic oceans.

from typing import List
from collections import deque


class Solution:
    def pacificAtlantic(self, heights: List[List[int]]) -> List[List[int]]:
        m, n = len(heights), len(heights[0])
        # Queue for processing squares
        pacq = deque()
        atlq = deque()
        # Set for passing squares for each side
        pac = set()
        atl = set()

        #Traverse all rows
        for i in range(m):
            #Add left side pacific touching 
            pacq.append((i, 0))
            pac.add((i, 0))
            #Add right side, atlantic touching
            atlq.append((i, n-1))
            atl.add((i, n-1))
        # Traverse all columns
        for i in range(n):
            # Add top side, pacific touching
            pacq.append((0, i))
            pac.add((0, i))
            #Add bottom side, atlantic touching
            atlq.append((m-1, i))
            atl.add((m-1, i))

        # Traverse from the Pacific outwards for each currently passing square
        while pacq:
            r, c = pacq.popleft()
            # Check 4 directions from current square
            for i, j in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
                x, y = r+i, c+j
                # Skip adjacent places not on our board
                if x < 0 or x >= m or 0 > y or y >= n:
                    continue
                # Skip squares already passing
                # Skips square not >= to water path
                if heights[r][c] <= heights[x][y] and (x, y) not in pac:
                    pacq.append((x, y))
                    pac.add((x, y))

        # Traverse from the Atlantic outwards for each currently passing square
        while atlq:
            r, c = atlq.popleft()
            # Check 4 directions from current square
            for i, j in [(1, 0), (0, 1), (-1, 0), (0, -1)]:
                x, y = r+i, c+j
                # Skip adjacent places not on our board
                if x < 0 or x >= m or 0 > y or y >= n:
                    continue
                # Skip squares already passing
                # Skips square not >= to water path
                if heights[r][c] <= heights[x][y] and (x, y) not in atl:
                    atlq.append((x, y))
                    atl.add((x, y))
        return atl & pac



s = Solution()
print(s.pacificAtlantic([[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]))
print(s.pacificAtlantic([[1]]))