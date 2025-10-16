// Given the head of a singly linked list and two integers left and right where left <= right, reverse the nodes of the list from position left to position right, and return the reversed list.

//! Constraints:
// The number of nodes in the list is n.
// 1 <= n <= 500
// -500 <= Node.val <= 500
// 1 <= left <= right <= n

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

struct Solution;
impl Solution {
  pub fn reverse_between(
    head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
  ) -> Option<Box<ListNode>> {
    if head.is_none() {
      return head;
    }

    let mut holder = Box::new(ListNode { val: 0, next: head });
    let mut prev = &mut holder;

    // Move prev to the node before the reversal starts
    for _ in 1..left {
      prev = prev.next.as_mut().unwrap();
    }

    // Start reversing
    let mut curr = prev.next.take();
    let tail = curr.as_mut().map(|node| node as *mut Box<ListNode>).unwrap();
    for _ in left..=right {
      if let Some(mut node) = curr {
        let next = node.next.take();
        node.next = prev.next.take();
        prev.next = Some(node);
        curr = next;
      }
    }

    // Connect the tail to the remaining part
    unsafe {
      (*tail).next = curr;
    }

    holder.next
  }
}