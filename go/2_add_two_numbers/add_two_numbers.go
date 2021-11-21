package main

type ListNode struct {
	Val  int
	Next *ListNode
}

var empty = ListNode{0, nil}

func safeNode(l *ListNode) ListNode {
	if l != nil {
		return *l
	}
	return empty
}

func addTwoNumbers(list1 *ListNode, list2 *ListNode) *ListNode {
	head := &ListNode{Val: 0, Next: nil}

	n := head
	up := 0
	l1, l2 := list1, list2

	for l1 != nil || l2 != nil {
		s1, s2 := safeNode(l1), safeNode(l2)

		sum := s1.Val + s2.Val + up
		up = sum / 10
		n.Next = &ListNode{Val: sum % 10, Next: nil}

		l1, l2 = s1.Next, s2.Next
		n = n.Next
	}

	if up != 0 {
		n.Next = &ListNode{up, nil}
	}

	if head.Next == nil {
		return head
	}
	return head.Next
}
