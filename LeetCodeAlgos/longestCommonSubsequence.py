# Given two strings text1 and text2, return the length of their longest common subsequence. If there is no common subsequence, return 0.
# A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
# For example, "ace" is a subsequence of "abcde".
# A common subsequence of two strings is a subsequence that is common to both strings.

class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        if text1 == text2: return len(text1)

        n, m = len(text1), len(text2)
        mat = [[0]*n]*m

        for row in range(m):
            for col in range(n):
                prevRow = mat[row-1][col] if row > 0 else 0
                prevCol = mat[row][col-1] if col > 0 else 0

                if text2[row] == text1[col]:
                    mat[row][col] = max(mat[row-1][col-1]+1 if row and col else 1, prevRow, prevCol)
                else:
                    mat[row][col] = max(prevRow, prevCol)

        return mat[m-1][n-1]
    
s = Solution()
print(s.longestCommonSubsequence("abcde","ace"))
print(s.longestCommonSubsequence("abc","abc"))
print(s.longestCommonSubsequence("abc","def"))