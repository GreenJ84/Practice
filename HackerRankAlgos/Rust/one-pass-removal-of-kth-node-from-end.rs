// Given the head of a singly linked list and an integer k, remove the k-th node from the end in one traversal and return the new head. If k is invalid, return the original list.

/*
 * Complete the 'removeKthNodeFromEnd' function below.
 *
 * The function is expected to return an INTEGER_SINGLY_LINKED_LIST.
 * The function accepts following parameters:
 *  1. INTEGER_SINGLY_LINKED_LIST head
 *  2. INTEGER k
 */

/*
 * For your reference:
 *
 * SinglyLinkedListNode {
 *     data: i32,
 *     next: *mut SinglyLinkedListNode,
 * };
*/

struct SinglyLinkedListNode {
    data: i32,
    next: *mut SinglyLinkedListNode,
}
impl SinglyLinkedListNode {
    fn new(data: i32) -> Self {
        SinglyLinkedListNode {
            data,
            next: std::ptr::null_mut(),
        }
    }
    fn from_vec(vec: Vec<i32>) -> *const SinglyLinkedListNode {
        let mut head: *mut SinglyLinkedListNode = std::ptr::null_mut();
        let mut current: *mut SinglyLinkedListNode = std::ptr::null_mut();
        for &value in vec.iter() {
            let new_node = Box::into_raw(Box::new(SinglyLinkedListNode::new(value)));
            if head.is_null() {
                head = new_node;
                current = head;
            } else {
                unsafe {
                    (*current).next = new_node;
                }
                current = new_node;
            }
        }
        head
    }
}

// fn removeKthNodeFromEnd(head: *const SinglyLinkedListNode, k: i32) -> *const SinglyLinkedListNode {
fn remove_kth_node_from_end(
    head: *const SinglyLinkedListNode,
    k: i32,
) -> *const SinglyLinkedListNode {
    if head.is_null() || k < 0 {
        return head;
    }

    let mut fast = head;
    let mut slow = head as *mut SinglyLinkedListNode;

    for _ in 0..k {
        unsafe {
            if (*fast).next.is_null() {
                return head;
            }
            fast = (*fast).next;
        }
    }

    unsafe {
        if (*fast).next.is_null() {
            return (*head).next as *const SinglyLinkedListNode;
        }
        fast = (*fast).next;
    }
    unsafe {
        while !(*fast).next.is_null() {
            fast = (*fast).next;
            slow = (*slow).next;
        }
    }

    unsafe {
        (*slow).next = (*(*slow).next).next;
    }

    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let head = SinglyLinkedListNode::from_vec(vec![5, 6, 7, 8]);
        let k = 3;
        let new_head = remove_kth_node_from_end(head, k);
        // Convert the resulting linked list back to a vector for easy comparison
        let mut result_vec = Vec::new();
        let mut current = new_head;
        while !current.is_null() {
            unsafe {
                result_vec.push((*current).data);
                current = (*current).next;
            }
        }
        assert_eq!(result_vec, vec![6, 7, 8]);
    }
}
// Example

// Input

// head = [5, 6, 7, 8]
// k = 3
// Output

// [6, 7, 8]
// Explanation

// The list has 4 nodes.
// The k-th node from the end with k=3 is the 4th node from the end (value 5), which is the head. Removing it yields [6,7,8].
