package main

func lengthOfLongestSubstring(s string) int {
	chars := []byte(s)
	size := len(chars)

	buf := make(map[byte]struct{}, size)
	length, head, last := 0, 0, 0
	for head < size && last < size {
		if _, ok := buf[chars[last]]; !ok {
			buf[chars[last]] = struct{}{}
			last++

			l := last - head
			if length < l {
				length = l
			}
		} else {
			delete(buf, chars[head])
			head++
		}
	}

	return length
}
