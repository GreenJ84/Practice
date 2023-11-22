# A valid cut in a circle can be:

# A cut that is represented by a straight line that touches two points on the edge of the circle and passes through its center, or
# A cut that is represented by a straight line that touches one point on the edge of the circle and its center.
# Some valid and invalid cuts are shown in the figures below.

class Solution:
    def numberOfCuts(self, n: int) -> int:
        if n == 1: return 0
        return n if n % 2 == 1 else n // 2