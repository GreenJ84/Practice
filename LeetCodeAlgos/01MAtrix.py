# Given an m x n binary matrix mat, return the distance of the nearest 0 for each cell.
# The distance between two adjacent cells is 1.

from typing import List

class Solution:
    def updateMatrix(self, mat: List[List[int]]) -> List[List[int]]:
        rows, cols = len(mat), len(mat[0])

        for i in range(rows):
            for j in range(cols):
                if mat[i][j] != 0:
                    left = mat[i][j-1] if j > 0 else float('inf')
                    top = mat[i-1][j] if i > 0 else float('inf')
                    mat[i][j] = 1 + min(left,top)

        for i in range(rows-1,-1,-1):
            for j in range(cols-1,-1,-1):
                if mat[i][j] != 0:
                    right = mat[i][j + 1] if j < cols -1 else float('inf')
                    down = mat[i +1 ][j] if i < rows - 1 else float('inf')
                    mat[i][j] = min(mat[i][j] , 1 + min(right, down))
        return mat 


# class Solution:
#     def updateMatrix(self, mat: List[List[int]]) -> List[List[int]]:
#         for y in range(len(mat)):
#             for x in range(len(mat[0])):
#                 if mat[y][x] == 0:
#                     continue
#                 else:
#                     t = checkDist(y-1, x, mat, 1)
#                     b = checkDist(y+1, x, mat, 1)
#                     l = checkDist(y, x-1, mat, 1)
#                     r = checkDist(y, x+1, mat, 1)
#                     mat[y][x] = min(min(t,b), min(l,r))
#         return mat

# def checkDist(y, x, mat, dist):
#     if y==-1 or y==len(mat) or x==-1 or x==len(mat[0]):
#         return float('inf')
#     elif mat[y][x]==0:
#         return dist
#     else:
#         return min(min(checkDist(y-1, x, mat, 1),checkDist(y+1, x, mat, 1)), min(checkDist(y, x-1, mat, 1),checkDist(y, x+1, mat, 1)))

s = Solution()
print(s.updateMatrix([[0,0,0],[0,1,0],[0,0,0]]))
print(s.updateMatrix([[0,0,0],[0,1,0],[1,1,1]]))
print(s.updateMatrix([[0,1,1],[1,1,1],[1,1,1]]))