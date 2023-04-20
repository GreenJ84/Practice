# Given a positive integer n, find and return the longest distance between any two adjacent 1's in the binary representation of n. If there are no two adjacent 1's, return 0.
# Two 1's are adjacent if there are only 0's separating them (possibly no 0's). The distance between two 1's is the absolute difference between their bit positions. For example, the two 1's in "1001" have a distance of 3.

class Solution:
    def binaryGap(self, n: int) -> int:
        bin_string = bin(n)[2:]

        last_index = 0
        max_dist = 0
        for i in range(len(bin_string)):
            if bin_string[i] == "1":
                if last_index >= 0:
                    max_dist = max(max_dist, i-last_index)
                last_index = i
        return max_dist

s = Solution()
print(s.binaryGap(22))
print(s.binaryGap(8))
print(s.binaryGap(5))