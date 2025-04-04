// Common utilities and data structures can go here

// For example, you might want to define a ListNode struct for linked list problems
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

// Add more shared utilities as needed 