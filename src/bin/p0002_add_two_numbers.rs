#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy_head = ListNode::new(0);
    let mut p = l1;
    let mut q = l2;
    let mut current = &mut dummy_head;
    let mut carry = 0;

    while p.is_some() || q.is_some() {
        let x = p.as_ref().map_or(0, |node| node.val);
        let y = q.as_ref().map_or(0, |node| node.val);
        let sum = carry + x + y;
        carry = sum / 10;
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();

        if p.is_some() {
            p = p.and_then(|node| node.next);
        }
        if q.is_some() {
            q = q.and_then(|node| node.next);
        }
    }

    if carry > 0 {
        current.next = Some(Box::new(ListNode::new(carry)));
    }

    dummy_head.next
}
