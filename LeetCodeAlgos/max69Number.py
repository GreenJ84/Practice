# You are given a positive integer num consisting only of digits 6 and 9.
# Return the maximum number you can get by changing at most one digit (6 becomes 9, and 9 becomes 6).

class Solution:
    def maximum69Number (self, num):
        num = str(num)
        num = num.replace('6','9',1)
        return int(num)


s = Solution()
print(s.maximum69Number(9669))
print(s.maximum69Number(9996))
print(s.maximum69Number(9999))