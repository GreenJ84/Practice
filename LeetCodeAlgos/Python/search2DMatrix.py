# Write an efficient algorithm that searches for a value target in an m x n integer matrix matrix. This matrix has the following properties:
# Integers in each row are sorted from left to right.
# The first integer of each row is greater than the last integer of the previous row.


from typing import List

# Better runtime and memory performance
class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        def levelSearch(mat, start, end):
            while start <= end:
                mid = (start+end)//2
                if mid >= len(mat):
                    return False
                if mat[mid][0] == target or mat[mid][-1] == target: 
                    return True
                elif mat[mid][0] > target:
                    end = mid-1
                elif mat[mid][-1] < target:
                    start = mid+1
                else:
                    return binarySearch(mat[mid], 0, len(mat[mid]))
            return False

        def binarySearch(arr, start, end):
            while start <= end:
                mid = (start+end)//2
                if arr[mid] == target: 
                    return True
                elif arr[mid] > target:
                    end = mid-1
                else:
                    start = mid+1
            return False

        return levelSearch(matrix, 0, len(matrix))

# class Solution:
#     def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
#         m, n, o = len(matrix), len(matrix[0]), 0

#         for i in range(0, m):
#             if i+1 == m:
#                 o = i
#                 break
#             elif matrix[i+1][0] > target:
#                 o = i
#                 break
#         for j in range(0, n):
#             if matrix[o][j] == target:
#                 return True
#         return False

s = Solution()
# print(s.searchMatrix([[1,3,5,7],[10,11,16,20],[23,30,34,60]], 3))
# print(s.searchMatrix([[1,3,5,7],[10,11,16,20],[23,30,34,60]], 13))
# print(s.searchMatrix([[1,2,3,4,7],[9,10,11,12,19],[25, 26, 39, 40, 70]], 5))
print(s.searchMatrix([[1, 1]], 2))