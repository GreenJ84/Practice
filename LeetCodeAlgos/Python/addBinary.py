# Given two binary strings a and b, return their sum as a binary string.

#* Highest performance Solution (87% runtime/ 98% memory)
class Solution:
    def addBinary(self, a: str, b: str) -> str:
        # Convert both binary strings to Integers at Base 2 with #! int(_, 2)
        # Add the two integers
        # Convert to a binary string with #! bin() -> returns '0b1010101010101010101010'
        # Slice string to remove the begining #! '0b'
        return bin(int(a, 2)+int(b, 2))[2:]


## Condenced variable for better runtime
# class Solution:
#     def addBinary(self, a: str, b: str) -> str:
#         res, sum = '', 0

#         for i in range(-1, -max(len(a), len(b))-1, -1):
#             if i >= -len(a):
#                 sum+=int(a[i])
#             if i >= -len(b):
#                 sum+=int(b[i])

#             res+=str(sum%2)
#             sum = sum//2

#         if sum == 1:
#             res+=str(sum)
#         res = ''.join(reversed(res))
#         return res

## First Brute Attempt
# class Solution:
#     def addBinary(self, a: str, b: str) -> str:
#         res, sum = '', 0
#         lenn = -max(len(a), len(b))

#         for i in range(-1, lenn-1, -1):
#             if i >= -len(a):
#                 sum+=int(a[i])

#             if i >= -len(b):
#                 sum+=int(b[i])

#             res+=str(sum%2)
#             sum = sum//2

#         if sum == 1:
#             res+=str(sum)
#         res = ''.join(reversed(res))
#         return res

s = Solution()
print(s.addBinary("11", "1"))
print(s.addBinary("1010", "1011"))