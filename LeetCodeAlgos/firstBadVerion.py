# You are a product manager and currently leading a team to develop a new product. Unfortunately, the latest version of your product fails the quality check. Since each version is developed based on the previous version, all the versions after a bad version are also bad.
# Suppose you have n versions [1, 2, ..., n] and you want to find out the first bad one, which causes all the following ones to be bad.
# You are given an API bool isBadVersion(version) which returns whether version is bad. Implement a function to find the first bad version. You should minimize the number of calls to the API.
def isBadVersion(version: int) -> bool:
    if version >= 1:
        return True
    else:
        return False

class Solution:
    def firstBadVersion(self, n: int) -> int:
        if n == 1:
            return n
        start, end = 1, n
        while start <= end:
            x = (start+end)//2
            if isBadVersion(end):
                    end -= 1
            else:
                return end+1
            if isBadVersion(x) and x < end:
                end = x
            elif not isBadVersion(x):
                start = x
        return start

s = Solution()
# print(s.firstBadVersion(5))
print(s.firstBadVersion(3))
# print(s.firstBadVersion(2126753390))
