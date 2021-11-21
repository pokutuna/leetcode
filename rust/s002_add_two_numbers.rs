// https://leetcode.com/problems/add-two-numbers/

struct Solution {}

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

// ---

impl Solution {
  pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(0));

    let mut n = &mut head;
    let mut pair = (l1, l2);
    let mut up = 0;

    loop {
      match pair {
        (None, None) => {
          if up > 0 {
            n.next = Some(Box::new(ListNode::new(up)));
          };
          break;
        }
        (o1, o2) => {
          let sum = o1.as_ref().map_or(0, |n| n.val) + o2.as_ref().map_or(0, |n| n.val) + up;
          let (sum, u) = if sum > 9 { (sum - 10, 1) } else { (sum, 0) };
          up = u;
          n.next = Some(Box::new(ListNode::new(sum)));
          n = n.next.as_mut().unwrap();
          pair = (o1.and_then(|n| n.next), o2.and_then(|n| n.next));
        }
      }
    }
    head.next
  }
}
// ---

#[cfg(test)]
mod tests {
  use super::*;

  fn vec_to_list(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(0));
    let mut tmp = &mut head;
    for i in v.iter() {
      tmp.next = Some(Box::new(ListNode::new(*i)));
      tmp = tmp.next.as_mut().unwrap();
    }
    return head.next;
  }

  #[test]
  fn test_2() {
    assert_eq!(
      Solution::add_two_numbers(vec_to_list(vec![2, 4, 3]), vec_to_list(vec![5, 6, 4])),
      vec_to_list(vec![7, 0, 8])
    );
    assert_eq!(
      Solution::add_two_numbers(vec_to_list(vec![0]), vec_to_list(vec![0])),
      vec_to_list(vec![0])
    );

    assert_eq!(
      Solution::add_two_numbers(
        vec_to_list(vec![9, 9, 9, 9, 9, 9, 9]),
        vec_to_list(vec![9, 9, 9, 9])
      ),
      vec_to_list(vec![8, 9, 9, 9, 0, 0, 0, 1])
    );
  }
}
