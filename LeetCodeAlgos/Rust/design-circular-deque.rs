// Design your implementation of the circular double-ended queue (deque).

// Implement the MyCircularDeque class:

// MyCircularDeque(int k) Initializes the deque with a maximum size of k.
// boolean insertFront() Adds an item at the front of Deque. Returns true if the operation is successful, or false otherwise.
// boolean insertLast() Adds an item at the rear of Deque. Returns true if the operation is successful, or false otherwise.
// boolean deleteFront() Deletes an item from the front of Deque. Returns true if the operation is successful, or false otherwise.
// boolean deleteLast() Deletes an item from the rear of Deque. Returns true if the operation is successful, or false otherwise.
// int getFront() Returns the front item from the Deque. Returns -1 if the deque is empty.
// int getRear() Returns the last item from Deque. Returns -1 if the deque is empty.
// boolean isEmpty() Returns true if the deque is empty, or false otherwise.
// boolean isFull() Returns true if the deque is full, or false otherwise.

use std::{
  cell::RefCell,
  rc::{Rc, Weak}
};

struct DNode {
  pub val: i32,
  pub prev: Option<Weak<RefCell<DNode>>>,
  pub next: Option<Rc<RefCell<DNode>>>,
}
impl DNode {
  pub fn new(val: i32) -> Self {
    Self {
      val,
      prev: None,
      next: None
    }
  }
}

struct MyCircularDeque {
  size: i32,
  capacity: i32,
  head: Option<Rc<RefCell<DNode>>>,
  tail: Option<Rc<RefCell<DNode>>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {

    fn new(k: i32) -> Self {
        Self {
          head: None,
          tail: None,
          size: 0,
          capacity: k
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
      if self.is_full() { return false; }

      let node = Rc::new(RefCell::new(DNode::new(value)));
      match self.head.take() {
        Some(head) => {
          head.borrow_mut().prev = Some(Rc::downgrade(&node));
          node.borrow_mut().next = Some(Rc::clone(&head));
          self.head = Some(node);
        },
        None =>  {
          self.head = Some(Rc::clone(&node));
          self.tail = Some(node);
        }
      }
      self.size += 1;
      true
    }

    fn insert_last(&mut self, value: i32) -> bool {
      if self.is_full() { return false; }

      let node = Rc::new(RefCell::new(DNode::new(value)));
      match self.tail.take() {
        Some(tail) => {
          tail.borrow_mut().next = Some(Rc::clone(&node));
          node.borrow_mut().prev = Some(Rc::downgrade(&tail));
          self.tail = Some(node);
        },
        None =>  {
          self.head = Some(Rc::clone(&node));
          self.tail = Some(node);
        }
      }
      self.size += 1;
      true
    }

    fn delete_front(&mut self) -> bool {
      if let Some(head) = self.head.take() {
        if Rc::ptr_eq(&head, self.tail.as_ref().unwrap()) {
          // Clear tail
          self.tail = None;
        } else {
          // Traverse forward one, and set the new head
          self.head = head.borrow_mut().next.take();
          // Cut straggling old (deleted) head
          self.head.as_ref().unwrap().borrow_mut().prev = None;
        }
        self.size -= 1;
        return true;
      }
      false
    }

    fn delete_last(&mut self) -> bool {
      if let Some(tail) = self.tail.take() {
        if Rc::ptr_eq(&tail, self.head.as_ref().unwrap()) {
          // Clear head
          self.head = None;
        } else {
          // Traverse back one, upgrade the reference, and set the new tail
          self.tail = tail.borrow_mut().prev.take().and_then(|weak| weak.upgrade());
          // Cut straggling old (deleted) tail
          self.tail.as_ref().unwrap().borrow_mut().next = None;
        }
        self.size -= 1;
        return true;
      }
      false
    }

    fn get_front(&self) -> i32 {
      if let Some(head) = &self.head {
        return head.borrow().val;
      }
      -1
    }

    fn get_rear(&self) -> i32 {
      if let Some(tail) = &self.tail {
        return tail.borrow().val;
      }
      -1
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}