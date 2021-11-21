package main

import (
	"testing"

	"gopkg.in/go-playground/assert.v1"
)

func TestLengthOfLogestSubstring(t *testing.T) {
	cases := []struct {
		input  string
		expect int
	}{
		{"abcabcbb", 3},
		{"bbbbb", 1},
		{"pwwkew", 3},
		{"abcabcbdefe", 5},
		{"", 0},
		{"ggububgvfk", 6},
		{" ", 1},
	}

	for _, c := range cases {
		assert.Equal(t, lengthOfLongestSubstring(c.input), c.expect)
	}
}
