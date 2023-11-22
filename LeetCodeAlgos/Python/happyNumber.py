# Write an algorithm to determine if a number n is happy.
# A happy number is a number defined by the following process:
# Starting with any positive integer, replace the number by the sum of the squares of its digits.
# Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
# Those numbers for which this process ends in 1 are happy.
# Return true if n is a happy number, and false if not.

class Solution(object):
    def isHappy(self, n):
        nStr = str(n)
        count = 20

        while(True):
            x = 0
            for i in nStr:
                x += int(i)**2
            count-=1
            if count < 0:
                return False
            if x == 1: return True
            nStr = str(x)

# class Solution(object):
#     def isHappy(self, n):
#         if n < 0:
#             return False

#         nStr = str(n)
#         x = 0
#         count = 20
#         while(x != 1):
#             x = 0
#             for i in nStr:
#                 x += int(i)**2
#             count-=1
#             if count < 0:
#                 return False
#             nStr = str(x)
#         return True


s = Solution()
print(s.isHappy(19))
print(s.isHappy(2)) 