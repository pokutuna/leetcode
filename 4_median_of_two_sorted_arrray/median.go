package main

// Naive implementation than the solution on website
//
// Runtime: 12 ms, faster than 86.49% of Go online submissions for Median of Two Sorted Arrays.
// Memory Usage: 5.6 MB, less than 65.00% of Go online submissions for Median of Two Sorted Arrays.
func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	l1, l2 := len(nums1), len(nums2)
	center, odd := (l1+l2)/2, (l1+l2)%2 == 1

	c, i1, i2, ma, mb := 0, 0, 0, 0, 0
	for c <= center {
		mb = ma
		if i1 < l1 && i2 < l2 {
			if nums1[i1] < nums2[i2] {
				ma = nums1[i1]
				i1++
			} else {
				ma = nums2[i2]
				i2++
			}
		} else if i1 < l1 {
			ma = nums1[i1]
			i1++
		} else {
			ma = nums2[i2]
			i2++
		}
		c++
	}

	if odd {
		return float64(ma)
	}
	return float64(ma+mb) / 2
}
