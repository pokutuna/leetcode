package main

import (
	"testing"

	"gopkg.in/go-playground/assert.v1"
)

func TestFindMedianSortedArrays(t *testing.T) {
	cases := []struct {
		a      []int
		b      []int
		expect float64
	}{
		{[]int{1, 3}, []int{2}, 2},
		{[]int{1, 2}, []int{3, 4}, 2.5},
		{[]int{1}, []int{}, 1},
		{[]int{}, []int{2}, 2},
		{[]int{}, []int{2, 3, 4}, 3},
		{[]int{}, []int{2, 3, 4, 5}, 3.5},
	}

	f := findMedianSortedArrays
	for _, c := range cases {
		assert.Equal(t, f(c.a, c.b), c.expect)
	}
}
