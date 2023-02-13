# Given a positive integer n, generate an n x n matrix filled with elements from 1 to n2 in spiral order.

from typing import List


class Solution:
    def generateMatrix(self, n: int) -> List[List[int]]:
        arr = [[1 for i in range(n)] for i in range(n)]
        curr = 1
        i, j, step = 0, -1, 0
        while curr <= n*n:
            for _ in range(-1+step, n-1-step):
                j+=1
                arr[i][j] = curr
                curr+=1
                if curr > n*n: break
            for _ in range(0+step, n-1-step):
                i+=1
                arr[i][j] = curr
                curr+=1
                if curr > n*n: break
            for _ in range(n-1-step, 0+step, -1):
                j-=1
                arr[i][j] = curr
                curr+=1
                if curr > n*n: break
            for _ in range(n-1-step, 1+step, -1):
                i-=1
                arr[i][j] = curr
                curr+=1
                if curr > n*n: break
            step+=1
        return arr

s = Solution()
print(s.generateMatrix(1))
print(s.generateMatrix(2))
print(s.generateMatrix(3))
print(s.generateMatrix(4))
print(s.generateMatrix(5))
print(s.generateMatrix(6))
print(s.generateMatrix(8))
print(s.generateMatrix(10))
print(s.generateMatrix(12))
