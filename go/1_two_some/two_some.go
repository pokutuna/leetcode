package main

func twoSum(nums []int, target int) []int {
	m := make(map[int]int, len(nums))
	for i, n := range nums {
		m[n] = i
	}
	for i, n := range nums {
		if v, ok := m[target-n]; ok && v != i {
			return []int{i, v}
		}
	}
	return nil
}
