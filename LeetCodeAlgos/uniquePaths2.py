class Solution(object):
    def uniquePathsWithObstacles(self, obstacleGrid):
        obGr = obstacleGrid
        m = len(obGr)
        n = len(obGr[0])
        # Base Cases
        if m == n <= 1:
            return 0
        if m == 1:
            if 1 not in n:
                return 1
            else: 
                return 0

        if n == 1:
            for i in obGr:
                if 1 in i:
                    return 0
            return 1

        #! Change all obsticles negative
        s = 0
        for i in obGr:
            obGr[s] = [ -1 if j > 0 else j for j in i]
            s+=1

        #! Set edge Paths to 1 as the are unique by themselves and edges
        # Set Endpoint to 1
        obGr[m-1][n-1] = 1
        # Set Right edge to 1s
        for item in range(n):
            print(obGr[m-1][item])
            if obGr[m-1][item] == 0:
                obGr[m-1][item] = 1
        # Set Bottom edge to 1s
        for row in range(m):
            print(obGr[row][n-1])
            if obGr[row][n-1] == 0:
                obGr[row][n-1] = 1

        # From the second to last item in the second to last row, start to come backwards to the robot checking bottom and right adjacents
        for row in range(m-2, -1, -1):
            for item in range(n-2, -1, -1):
                current = obGr[row][item]
                bottom = obGr[row+1][item]
                right = obGr[row][item+1]

                if current == 0:
                    if bottom <= 0:
                        obGr[row][item] = right
                    elif right <= 0:
                        obGr[row][item] = bottom
                    else:
                        obGr[row][item] = bottom + right

        # Return maximum of routes the robot can take when you get back to him
        return obGr[0][0]

solution = Solution()
x = solution.uniquePathsWithObstacles([[0,0,0],[0,1,0],[0,0,0]])
print(x)