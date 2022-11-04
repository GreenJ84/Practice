class Solution(object):
    def isHappy(self, n):
        if n < 0:
            return False

        nStr = str(n)
        x = 0
        count = 20
        while(x != 1):
            x = 0
            for i in nStr:
                x += int(i)**2
            count-=1
            if count < 0:
                return False
            nStr = str(x)
        return True


s = Solution()
print(s.isHappy(19))
print(s.isHappy(2)) 