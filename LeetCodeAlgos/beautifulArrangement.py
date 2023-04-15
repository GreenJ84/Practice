# Suppose you have n integers labeled 1 through n. A permutation of those n integers perm (1-indexed) is considered a beautiful arrangement if for every i (1 <= i <= n), either of the following is true:
    # perm[i] is divisible by i.
    # i is divisible by perm[i].
    # Given an integer n, return the number of the beautiful arrangements that you can construct.

class Solution:
    def countArrangement(self, n: int) -> int:
        def backtrack(curr, perm):
            nonlocal count
            if curr == n+1:
                count += 1
                return
            for next in range(1, n+1):
                if next not in perm and (curr % next == 0 or next % curr == 0):
                    backtrack(curr + 1, perm + [next])

        count = 0
        backtrack(1, [])
        return count
    
s = Solution()
print(s.countArrangement(2))
print(s.countArrangement(1))
print(s.countArrangement(3))