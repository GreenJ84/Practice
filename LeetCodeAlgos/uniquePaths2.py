class Solution(object):
    def uniquePathsWithObstacles(self, obstacleGrid):
        obGr = obstacleGrid
        m = len(obGr)
        n = len(obGr[0])
        # Base Cases
        if m < 1 | n < 1:
            return 0
        if obGr[m-1][n-1] | obGr[0][0] == 1:
            return 0
        if m == n == 1:
            return 1
        if m == 1:
            if 1 not in obGr[0]:
                return 1
            else: 
                return 0
        if n == 1:
            for i in obGr:
                if 1 in i:
                    return 0
            return 1

        s = 0
        for i in obGr:
            obGr[s] = [ -1 if j > 0 else j for j in i]
            s+=1

        obGr[m-1][n-1] = 1

        for item in range(n-1, -1, -1):
            print(obGr[m-1][item])
            if obGr[m-1][item] == 0:
                obGr[m-1][item] = 1
            elif obGr[m-1][item] == -1:
                break

        for row in range(m-1, -1, -1):
            print(obGr[row][n-1])
            if obGr[row][n-1] == 0:
                obGr[row][n-1] = 1
            elif obGr[row][n-1] == -1:
                break

        for row in range(m-2, -1, -1):
            for item in range(n-2, -1, -1):
                current = obGr[row][item]
                bottom = obGr[row+1][item]
                right = obGr[row][item+1]

                if current == 0:
                    if bottom & right <= 0:
                        obGr[row][item] = -1
                    elif bottom <= 0:
                        obGr[row][item] = right
                    elif right <= 0:
                        obGr[row][item] = bottom
                    else:
                        obGr[row][item] = bottom + right

        if obGr[0][0] > 0:
            return obGr[0][0]
        else: 
            return 0

solution = Solution()
print(solution.uniquePathsWithObstacles([[0,0],[1,1],[0,0]]))