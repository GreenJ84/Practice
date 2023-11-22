class Solution:
    def hammingWeight(self, n: int) -> int:
        if n == 0:
            return 0
        elif n == 1:
            return 1
        else:
            res = 0
            while(n!=1):
                if n%2 ==1:
                    res+=1
                    n-=1
                if n==2:
                    res+=1
                n/=2
            return res

s = Solution()
# print(s.hammingWeight(11))
# print(s.hammingWeight(128))
print(s.hammingWeight(4294967293))

