# Given a positive integer n, find the smallest integer which has exactly the same digits existing in the integer n and is greater in value than n. If no such positive integer exists, return -1.
# Note that the returned integer should fit in 32-bit integer, if there is a valid answer but it does not fit in 32-bit integer, return -1.

class Solution:
    def nextGreaterElement(self, n: int) -> int:
        m = list(str(n))
        stack = []
        def check(t, b):
            max = int(t)
            for i in stack:
                i = int(i)
                if i < max and i > int(b):
                    max = i
            return str(max)


        for i in range(len(m)-1, 0, -1):
            if m[i-1]<m[i]:
                if stack:
                    x = check(m[i], m[i-1])
                    stack.append(m[i-1])
                    stack.append(m[i])
                    m[i-1]=x
                    stack.remove(x)
                else:
                    stack.append(m[i-1])
                    m[i-1]=m[i]
                break
            stack.append(m[i])
        j = -1
        while stack:
            m[j] = max(stack)
            stack.remove(max(stack))
            j-=1
        m = int("".join(m))
        return m if (m > n and m <= 2147483647) else -1

#! Attempt 1. 60% of test cases passed
# class Solution:
#     def nextGreaterElement(self, n: int) -> int:
#         m = list(str(n))
#         res = []
#         for i in range(len(m)-1, 0, -1):
#             for j in range(i-1, -1, -1):
#                 if m[j]<m[i]:
#                     m[j], m[i] = m[i], m[j]
#                     res.append(int(''.join(m)))
#                     m = list(str(n))
#         if res:
#             return min(res)
#         return -1
    
s = Solution()
print(s.nextGreaterElement(12))
print(s.nextGreaterElement(21))
print(s.nextGreaterElement(230241))
print(s.nextGreaterElement(2147483476))
print(s.nextGreaterElement(12443322))