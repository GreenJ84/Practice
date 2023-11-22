// You are given the heads of two sorted linked lists list1 and list2.

// Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.

// Return the head of the merged linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
        next: None,
        val
        }
    }
}

struct Solution {}
type T = Option<Box<ListNode>>;
impl Solution {
    pub fn merge_two_lists(list1: T, list2: T) -> T {
        match (list1, list2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(list1), Some(list2)) => {
                if list1.val < list2.val {
                    Some(Box::new(ListNode {
                        val: list1.val,
                        next: Self::merge_two_lists(list1.next, Some(list2))
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: list2.val,
                        next: Self::merge_two_lists(Some(list1), list2.next)
                    }))
                }
            },
        }
    }
}

fn main() {}