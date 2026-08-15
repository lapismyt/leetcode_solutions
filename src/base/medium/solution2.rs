use crate::base::Solution;

// Definition for singly-linked list.
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

fn sum_list_nodes(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    offset: i32,
) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && offset == 0 {
        return None;
    }

    let (l1_num, l1_child) = l1.map_or((0, None), |l1| (l1.val, l1.next));
    let (l2_num, l2_child) = l2.map_or((0, None), |l2| (l2.val, l2.next));

    let mut sum = l1_num + l2_num + offset;
    let mut offset = 0;

    if sum >= 10 {
        offset = 1;
        sum -= 10;
    }

    Some(Box::new(ListNode {
        val: sum,
        next: sum_list_nodes(l1_child, l2_child, offset),
    }))
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        sum_list_nodes(l1, l2, 0)
    }
}
