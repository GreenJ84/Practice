# Given the coordinates of two rectilinear rectangles in a 2D plane, return the total area covered by the two rectangles.
# The first rectangle is defined by its bottom-left corner (ax1, ay1) and its top-right corner (ax2, ay2).
# The second rectangle is defined by its bottom-left corner (bx1, by1) and its top-right corner (bx2, by2).

class Solution:
    def computeArea(self, ax1: int, ay1: int, ax2: int, ay2: int, bx1: int, by1: int, bx2: int, by2: int) -> int:
        # Square 1 Area: Length times height
        sq1 = (ax2-ax1)*(ay2-ay1)
        # Square 2 Area: Length times height
        sq2 = (bx2-bx1)*(by2-by1)
        # Top-Bottom range of Overlap if any
        h = max(min(ay2, by2)-max(ay1, by1), 0)
        # Left-Right range of Overlap if any
        l = max(min(ax2, bx2)-max(ax1, bx1), 0)
        # Calc Overlap if any
        # Return Square 1 Area + Square 2 Area - Overlap if any
        return sq1+sq2-l*h

s = Solution()
print(s.computeArea(-2,-2,2,2,3,3,4,4))
# print(s.computeArea(-3,0,3,4,0,-1,9,2))