package main

import (
	"fmt"
	"testing"

	"gopkg.in/go-playground/assert.v1"
)

func makeListNode(nums []int) *ListNode {
	list := &ListNode{
		Val:  nums[0],
		Next: nil,
	}

	n := list
	for _, v := range nums[1:] {
		n.Next = &ListNode{Val: v, Next: nil}
		n = n.Next
	}

	return list
}

func printListNode(list *ListNode) {
	for list != nil {
		fmt.Print(list.Val)
		list = list.Next
	}
	fmt.Print("\n")
}

func TestAddTwoNumbers(t *testing.T) {
	n1 := makeListNode([]int{2, 4, 3})
	n2 := makeListNode([]int{5, 6, 4})
	assert.Equal(t, addTwoNumbers(n1, n2), makeListNode([]int{7, 0, 8}))
}

func TestAddTwoNumbersWithZeros(t *testing.T) {
	n1 := makeListNode([]int{0})
	n2 := makeListNode([]int{0})
	assert.Equal(t, addTwoNumbers(n1, n2), makeListNode([]int{0}))
}

func TestAddTwoNumbersWithUp(t *testing.T) {
	n1 := makeListNode([]int{5})
	n2 := makeListNode([]int{5})
	assert.Equal(t, addTwoNumbers(n1, n2), makeListNode([]int{0, 1}))
}

func TestAddTwoNumbersDifferentLength(t *testing.T) {
	n1 := makeListNode([]int{1, 8})
	n2 := makeListNode([]int{0})
	assert.Equal(t, addTwoNumbers(n1, n2), makeListNode([]int{1, 8}))
}

func TestAddTwoNumbersUp2(t *testing.T) {
	n1 := makeListNode([]int{1})
	n2 := makeListNode([]int{9, 9})
	assert.Equal(t, addTwoNumbers(n1, n2), makeListNode([]int{0, 0, 1}))
}

func compose(l *ListNode) int {
	n := 0
	factor := 1

	for true {
		n = n + l.Val*factor
		if l.Next != nil {
			l = l.Next
			factor = factor * 10
		} else {
			break
		}
	}

	return n
}

func decompose(num int) *ListNode {
	if num == 0 {
		return &ListNode{0, nil}
	}

	list := &ListNode{}

	l, n := list, num
	for n != 0 {
		div, mod := n/10, n%10
		l.Next = &ListNode{Val: mod, Next: nil}
		l = l.Next
		n = div
	}

	return list.Next
}
