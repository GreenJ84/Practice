# We are playing the Guess Game. The game is as follows:
# I pick a number from 1 to n. You have to guess which number I picked.
# Every time you guess wrong, I will tell you whether the number I picked is higher or lower than your guess.
# You call a pre-defined API int guess(int num), which returns three possible results:
# -1: Your guess is higher than the number I picked (i.e. num > pick).
# 1: Your guess is lower than the number I picked (i.e. num < pick).
# 0: your guess is equal to the number I picked (i.e. num == pick).
# Return the number that I picked.

class Solution:
    def guessNumber(self, n: int, pick:int) -> int:
        start = 0
        while start <= n:
            y = (n-start)//2+start
            x = guess(y, pick)
            if x == 0:
                return y
            elif x == 1:
                start = y+1
            elif x == -1:
                n = y-1

def guess(num: int, pick:int) -> int:
    if num == pick:
        return 0
    return 1 if num < pick else -1

s = Solution()
print(s.guessNumber(10, 6))
print(s.guessNumber(1, 1))
print(s.guessNumber(2, 1))