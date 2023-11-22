# Assume you are an awesome parent and want to give your children some cookies. But, you should give each child at most one cookie.
# Each child i has a greed factor g[i], which is the minimum size of a cookie that the child will be content with; and each cookie j has a size s[j]. If s[j] >= g[i], we can assign the cookie j to the child i, and the child i will be content. Your goal is to maximize the number of your content children and output the maximum number.

from typing import List


class Solution:
    def findContentChildren(self, g: List[int], s: List[int]) -> int:
        g.sort(reverse=True)
        s.sort(reverse=True)
        happyChild = 0
        x = y = 0

        while(x < len(g) and y < len(s)):
            if s[y] > g[x]:
                happyChild+=1
                s[y] -= g[x]
                g.pop(x)
                y+=1
            elif s[y] == g[x]:
                happyChild+=1
                g.pop(x)
                s.pop(y)
            else:
                x+=1
        return happyChild

s = Solution()
# print(s.findContentChildren([1,2,3], [1,1]))
# print(s.findContentChildren([1,2], [1,2,3]
# ))
print(s.findContentChildren([250,490,328,149,495,325,314,360,333,418,430,458],[376,71,228,110,215,410,363,135,508,268,494,288,24,362,20,5,247,118,152,393,458,354,201,188,425,167,220,114,148,43,403,385,512,459,71,425,142,102,361,102,232,203,25,461,298,437,252,364,171,240,233,257,305,346,307,408,163,216,243,261,137,319,33,91,116,390,139,283,174,409,191,338,123,231,101,458,497,306,400,513,175,454,273,88,169,250,196,109,505,413,371,448,12,193,396,321,466,526,276,276,198,260,131,322,65,381,204,32,83,431,81,108,366,188,443,331,102,72,496,521,502,165,439,161,257,324,348,176,272,341,230,323,124,13,51,241,186,329,70,387,93,126,159,370,292,16,211,327,431,26,70,239,379,368,215,501,382,299,481,163,100,488,259,524,481,87,118,112,110,425,295,352,62,162,19,404,301,163,389,13,383,43,397,165,385,274,59,499,136,309,301,345,381,124,394,492,96,243,4,297,153,9,210,291,33,450,202,313,138,214,308,239,129,154,354,289,484,388,351,339,337,161,97,185,190,498,348,242,38,217,343,170,269,465,514,89,366,447,166,52,33,436,268,3,74,505,403,302,513,69,439,68,72,403,33,130,466,417,186,339,328,237]))
