// Given the head of a singly linked list, return the middle node of the linked list.

// If there are two middle nodes, return the second middle node.



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
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cnt: i32 = 0;
        let mut run = head.clone();
        while let Some(node) = run {
            cnt += 1;
            run = node.next;
        }
        run = head;
        cnt = cnt / 2;
        for _ in 0..cnt as usize {
            run = run.unwrap().next;
        }
        run
    }
}