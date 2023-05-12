# Design a data structure to find the frequency of a given value in a given subarray.

# The frequency of a value in a subarray is the number of occurrences of that value in the subarray.
# Implement the RangeFreqQuery class:
    # RangeFreqQuery(int[] arr) Constructs an instance of the class with the given 0-indexed integer array arr.
    # int query(int left, int right, int value) Returns the frequency of value in the subarray arr[left...right].
    # A subarray is a contiguous sequence of elements within an array. arr[left...right] denotes the subarray that contains the elements of nums between indices left and right (inclusive).

from typing import List


class RangeFreqQuery:
    def __init__(self, arr: List[int]):
        self.arr = arr

    def query(self, left: int, right: int, value: int) -> int:
        freq = 0;
        for i in range(left, right + 1):
            if self.arr[i] == value:
                freq += 1
        return freq


# Your RangeFreqQuery object will be instantiated and called as such:
arr = [12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]
obj = RangeFreqQuery(arr)
param_1 = obj.query(1, 2, 4)
print(param_1)
param_2 = obj.query(0, 11, 33)
print(param_2)
assert param_1 == 1, f"Expected 1, got {param_1}"
assert param_2 == 2, f"Expected 2, got {param_2}"