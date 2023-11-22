class Solution:
    def findBall(self, grid):
        m, n = len(grid), len(grid[0])
        sol = []
        for b in range(0, n):
            x = b
            for i in range(0,m):
                # Gets stuck in cup or against wall
                match (grid[i][x]):
                    case 1:
                        if x+1 == n or grid[i][x+1] == -1:
                            sol.append(-1)
                            print(sol)
                            break
                        else: 
                            x+=1
                    case -1:
                        if x-1 < 0 or grid[i][x-1] == 1:
                            sol.append(-1)
                            print(sol)
                            break
                        else:
                            x-=1
                if i == m-1:
                    sol.append(x)
        return sol


s = Solution()
s.findBall([[1,1,1,-1,-1],[1,1,1,-1,-1],[-1,-1,-1,1,1],[1,1,1,1,-1],[-1,-1,-1,-1,-1]])

