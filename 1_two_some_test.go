package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestTwoSum(t *testing.T) {
	assert.Equal(t, twoSum([]int{2, 7, 11, 15}, 9), []int{0, 1})
	assert.Equal(t, twoSum([]int{2, 7, 11, 15}, 26), []int{2, 3})
	assert.Equal(t, twoSum([]int{3, 2, 4}, 6), []int{1, 2})
}
