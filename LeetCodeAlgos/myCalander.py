# You are implementing a program to use as your calendar. We can add a new event if adding the event will not cause a double booking.
# A double booking happens when two events have some non-empty intersection (i.e., some moment is common to both events.).
# The event can be represented as a pair of integers start and end that represents a booking on the half-open interval [start, end), the range of real numbers x such that start <= x < end.
# Implement the MyCalendar class:
    # MyCalendar() Initializes the calendar object.
    # boolean book(int start, int end) Returns true if the event can be added to the calendar successfully without causing a double booking. Otherwise, return false and do not add the event to the calendar.

class MyCalendar:
    def __init__(self):
        self.events = []

    def book(self, start: int, end: int) -> bool:
        for x in self.events:
            if (start<x[1] and end>x[1]) or (start<x[0] and end>x[0]) or (start<x[0] and end>x[1]):
                return False
        self.events.append([start,end])
        self.events = sorted(self.events)
        return True


obj = MyCalendar()
param_1 = obj.book(10, 20)
print(param_1)
param_2 = obj.book(15, 25)
print(param_2)
param_3 = obj.book(20, 30)
print(param_3)
