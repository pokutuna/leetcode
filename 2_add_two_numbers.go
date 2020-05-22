package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func safeNode(l *ListNode) *ListNode {
	if l != nil {
		return l
	}
	return &ListNode{0, nil}
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	list := &ListNode{0, nil}

	n := list
	up := 0
	for l1 != nil || l2 != nil {
		s1, s2 := safeNode(l1), safeNode(l2)
		sum := s1.Val + s2.Val + up
		n.Next = &ListNode{sum % 10, nil}
		up = sum / 10
		l1, l2 = s1.Next, s2.Next
		n = n.Next
	}
	if up != 0 {
		n.Next = &ListNode{up, nil}
	}

	if list.Next == nil {
		return list
	}
	return list.Next
}
