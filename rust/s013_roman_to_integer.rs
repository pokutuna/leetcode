impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let table: [(i32, &str); 13] = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut s: &str = &s;
        let mut num = 0;
        let mut i = 0;

        while s.len() > 0 {
            let (n, part) = table[i];
            if s.starts_with(part) {
                num += n;
                s = &s[(part.len())..];
            } else {
                i += 1;
            }
        }
        num
    }
}

// ------

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_13() {
        assert_eq!(Solution::roman_to_int("III".into()), 3);
        assert_eq!(Solution::roman_to_int("LVIII".into()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".into()), 1994);
    }
}
