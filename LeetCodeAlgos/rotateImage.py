# You are given an n x n 2D matrix representing an image, rotate the image by 90 degrees (clockwise).
# You have to rotate the image in-place, which means you have to modify the input 2D matrix directly. DO NOT allocate another 2D matrix and do the rotation.

from typing import List


class Solution:
    def rotate(self, matrix: List[List[int]]) -> None:
        x = zip(*matrix)
        for idx, row in enumerate(x):
            matrix[idx] = reversed(row)
        return matrix

s = Solution()
print(s.rotate([[1,2,3],[4,5,6],[7,8,9]]))
# print(s.rotate([[[1,2,3],[4,5,6],[7,8,9]]]))