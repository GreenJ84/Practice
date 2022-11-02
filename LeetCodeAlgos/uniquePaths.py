class Solution(object):
    def uniquePaths(self, m, n):
        # Base Cases
        if m == n <= 1:
            return 0
        if m == 1 | n == 1:
            return 1

        # Create matrix for robot
        matrix = [[0]*n]*m

        #! Set edge Paths to 1 as the are unique by themselves and edges
        # Set Endpoint to 1
        matrix[m-1][n-1] = 1
        # Set Right edge to 1s
        for item in range(n):
            matrix[m-1][item] = 1
        # Set Bottom edge to 1s
        for row in range(m):
            matrix[row][n-1] = 1

        # From the second to last ite in the second to last row, start to come backwards to the robot checking bottom and right adjacents
        for row in range(m-2, -1, -1):
            for item in range(n-2, -1, -1):
                matrix[row][item] = matrix[row+1][item] + matrix[row][item+1]

        # Return maximum of routes the robot can take when you get back to him
        return matrix[0][0]