// Given the head of a linked list and an integer val, remove all the nodes of the linked list that has Node.val == val, and return the new head.

// Definition for singly-linked list.
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
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut base = head.as_ref();
        while base.is_some() && base.unwrap().val == val {
            head = head.unwrap().next;
            base = head.as_ref();
        }
        if base.is_none() {
            return None;
        }

        let mut base = head.as_mut().unwrap();
        while base.next.is_some() {
            if base.next.as_ref().unwrap().val == val {
                base.next = base.next.as_mut().unwrap().next.take();
            } else {
                base = base.next.as_mut().unwrap();
            }
        }
        head
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = None;
        for idx in 0..nums.len() {
            let node = Box::new(ListNode::new(nums[idx]));
            if idx == 0 {
                head = Some(node);
                tail = head.as_mut();
            } else {
                tail.as_mut().unwrap().next = Some(node);
                tail = tail.unwrap().next.as_mut();
            }
        }
        head
    }

    #[test]
    fn test_example1() {
        let head = create_list(vec![1, 2, 6, 3, 4, 5, 6]);
        let val = 6;
        let result = Solution::remove_elements(head, val);
        let expected = create_list(vec![1, 2, 3, 4, 5]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example2() {
        let head = create_list(vec![]);
        let val = 1;
        let result = Solution::remove_elements(head, val);
        let expected = create_list(vec![]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example3() {
        let head = create_list(vec![7, 7, 7, 7]);
        let val = 7;
        let result = Solution::remove_elements(head, val);
        let expected = create_list(vec![]);
        assert_eq!(result, expected);
    }
}