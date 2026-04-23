// Write a function "deleteDuplicates" that removes consecutive duplicate nodes in-place, retaining only the first node of each code. Return the head of the resulting list.


struct SinglyLinkedListNode {
    data: i32,
    next: *mut SinglyLinkedListNode,
}

// fn deleteDuplicates(head: *const SinglyLinkedListNode) -> *const SinglyLinkedListNode {
fn delete_duplicates(head: *const SinglyLinkedListNode) -> *const SinglyLinkedListNode {
    let mut current = head as *mut SinglyLinkedListNode;
    while !current.is_null() {
        let mut next = unsafe { (*current).next };
        while !next.is_null() && unsafe { (*current).data } == unsafe { (*next).data } {
            next = unsafe { (*next).next };
        }
        unsafe { (*current).next = next };
        current = next;
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    fn linked_list_from_vec(vec: Vec<i32>) -> *const SinglyLinkedListNode {
        let mut head: *mut SinglyLinkedListNode = std::ptr::null_mut();
        let mut current: *mut SinglyLinkedListNode = std::ptr::null_mut();
        for &value in vec.iter() {
            let new_node = Box::into_raw(Box::new(SinglyLinkedListNode { data: value, next: std::ptr::null_mut() }));
            if head.is_null() {
                head = new_node;
                current = head;
            } else {
                unsafe { (*current).next = new_node };
                current = new_node;
            }
        }
        head
    }

    #[test]
    fn test_delete_duplicates() {
        let head = linked_list_from_vec(vec![1, 2, 2, 2, 3, 4, 4, 5]);
        let result = delete_duplicates(head);
        let mut current = result;
        let mut output = Vec::new();
        while !current.is_null() {
            output.push(unsafe { (*current).data });
            current = unsafe { (*current).next };
        }
        assert_eq!(output, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_delete_duplicates_empty() {
        let head = linked_list_from_vec(vec![]);
        let result = delete_duplicates(head);
        assert!(result.is_null());
    }

    #[test]
    fn test_delete_duplicates_no_duplicates() {
        let head = linked_list_from_vec(vec![1, 2, 3, 4, 5]);
        let result = delete_duplicates(head);
        let mut current = result;
        let mut output = Vec::new();
        while !current.is_null() {
            output.push(unsafe { (*current).data });
            current = unsafe { (*current).next };
        }
        assert_eq!(output, vec![1, 2, 3, 4, 5]);
    }
}

// Example

// Input

// head = [1, 2, 2, 2, 3, 4, 4, 5]
// Output

// [1, 2, 3, 4, 5]
// Explanation

// - Given 1→2→2→2→3→4→4→5. 
// - Start at 1 (next is 2, no skip). 
// - Move to 2; skip all consecutive 2's so that 2 links directly to 3 (removing two extra 2 nodes). 
// - Now list is 1→2→3→4→4→5.
// - Move to 3 (next is 4, no skip). 
// - At 4, skip the duplicate 4 so it links to 5. 
// - The resulting list is 1→2→3→4→5.