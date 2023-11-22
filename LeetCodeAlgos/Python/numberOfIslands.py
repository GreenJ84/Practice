# Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water), return the number of islands.
# An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.

from typing import List


class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        res = 0
        for i in range(0, len(grid)):
            for j in range(0, len(grid[0])):
                if grid[i][j] == "1":
                    res+=1
                    self.floodFill(grid, i, j, 0)
        return res

    def floodFill(self, image: List[List[int]], sr: int, sc: int, color: int) -> List[List[int]]:
        check = image[sr][sc]
        if check == color:
            return image
        image[sr][sc] = color
        if sr > 0 and image[sr-1][sc] == check:
            self.floodFill(image, sr-1, sc, color)
        if sc < len(image[0])-1 and image[sr][sc+1] == check:
            self.floodFill(image, sr, sc+1, color)
        if sr < len(image)-1 and image[sr+1][sc] == check:
            self.floodFill(image, sr+1, sc, color)
        if sc > 0 and image[sr][sc-1] == check:
            self.floodFill(image, sr, sc-1, color)
        return image