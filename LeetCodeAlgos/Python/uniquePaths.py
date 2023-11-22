
from itertools import product

# Increased performance and reduced code with Itertool product
class Solution(object):
    def uniquePaths(self, m, n):
        # Base Cases
        if m == n <= 1:
            return 0
        if m == 1 | n == 1:
            return 1

        # Create matrix for robot
        matrix = [[1]*n]*m
        for i, j in product(range(1, m), range(1, n)):
            matrix[i][j] = matrix[i-1][j] + matrix[i][j-1]
        return matrix[-1][-1]



# class Solution(object):
#     def uniquePaths(self, m, n):
#         # Base Cases
#         if m == n <= 1:
#             return 0
#         if m == 1 | n == 1:
#             return 1

#         # Create matrix for robot
#         matrix = [[0]*n]*m

#         # Set edge Paths to 1 as the are unique by themselves
#         # Set Endpoint to 1
#         matrix[m-1][n-1] = 1
#         # Set Right edge to 1s
#         for item in range(n):
#             matrix[m-1][item] = 1
#         # Set Bottom edge to 1s
#         for row in range(m):
#             matrix[row][n-1] = 1

#         # From the second to last item in the second to last row, start to come backwards to the robot checking bottom and right adjacents
#         for row in range(m-2, -1, -1):
#             for item in range(n-2, -1, -1):
#                 matrix[row][item] = matrix[row+1][item] + matrix[row][item+1]

#         # Return maximum of routes the robot can take when you get back to him
#         return matrix[0][0]

s = Solution()
print(s.uniquePaths(3, 7))
print(s.uniquePaths(3, 2))