# Design a system that manages the reservation state of n seats that are numbered from 1 to n.
# Implement the SeatManager class:
    # SeatManager(int n) Initializes a SeatManager object that will manage n seats numbered from 1 to n. All seats are initially available.
    # int reserve() Fetches the smallest-numbered unreserved seat, reserves it, and returns its number.
    # void unreserve(int seatNumber) Unreserves the seat with the given seatNumber.

class SeatManager:
    def __init__(self, n: int):
        self.tables = [False]*(n)
        self.next = [0]

    def reserve(self) -> int:
        self.tables[self.next[-1]] = True
        reserved = self.next.pop()+1
        if len(self.next) == 0:
            self.next.append(reserved)
            return reserved
        else:
            return reserved

    def unreserve(self, seatNumber: int) -> None:
        if self.tables[seatNumber-1] == True:
            self.tables[seatNumber-1] = False
            self.next.append(seatNumber-1)
            self.next.sort(reverse=True)


# Your SeatManager object will be instantiated and called as such:
obj = SeatManager(5)
print(obj.reserve())
print(obj.reserve())
obj.unreserve(2)
print(obj.reserve())
print(obj.reserve())
print(obj.reserve())
print(obj.reserve())
obj.unreserve(5)