package main

func scan(s []byte, from int) (map[byte]int, int) {
	buf := make(map[byte]int)
	idx := 0

	for i, char := range s[from:] {
		idx = i
		if _, ok := buf[char]; ok {
			return buf, idx
		} else {
			buf[char] = i + from
		}
	}

	return buf, idx + 1
}

func lengthOfLongestSubstring(s string) int {
	if s == "" {
		return 0
	}

	bytes := []byte(s)
	buf, length := scan(bytes, 0)
	scanned := length
	if scanned == len(bytes) {
		return length
	}

	next := buf[bytes[scanned]] + 1
	for next < len(bytes) {
		b, l := scan(bytes, next)
		scanned = next + l

		if l >= length {
			length, buf = l, b
			if scanned == len(bytes) {
				break
			}
			next = buf[bytes[scanned]] + 1
		} else {
			next = next + 1
		}
	}

	return length
}
