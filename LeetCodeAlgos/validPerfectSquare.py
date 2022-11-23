# Given a positive integer num, write a function which returns True if num is a perfect square else False.
# Follow up: Do not use any built-in library function such as sqrt.

class Solution:
    def isPerfectSquare(self, num: int) -> bool:
        if num==1:
            return True
        low, high = 0, num
        x = int((low+high)/2)
        while(low <= high):
            if x*x==num:
                return True
            elif x*x>num:
                high = x-1
            else:
                low = x+1
            x = int((low+high)/2)
        return False

s = Solution()
# print(s.isPerfectSquare(4))
# print(s.isPerfectSquare(5))
print(s.isPerfectSquare(9))
print(s.isPerfectSquare(64))