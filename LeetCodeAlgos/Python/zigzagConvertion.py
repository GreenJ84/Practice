# The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
    # P   A   H   N
    # A P L S I I G
    # Y   I   R
# And then read line by line: "PAHNAPLSIIGYIR"
# Write the code that will take a string and make this conversion given a number of rows:

class Solution:
    def convert(self, s: str, numRows: int) -> str:
        if len(s) <= numRows or numRows == 1:
            return s
        mat = [""]*numRows

        row = 0
        down = True
        for l in s:
            if down:
                mat[row] += l
                if row == numRows-1:
                    down = False
                    row -= 1
                    continue
                row += 1
            else:
                mat[row] += l
                if row == 0:
                    down = True
                    row += 1
                    continue
                row -= 1

        return "".join(mat)
    
s = Solution()
print(s.convert("PAYPALISHIRING", 3) == "PAHNAPLSIIGYIR")
print(s.convert("PAYPALISHIRING", 4) == "PINALSIGYAHRPI")
print(s.convert("A", 1) == "A")
print(s.convert("ABC", 1) == "ABC")