const TABLE: [(i32, &str); 13] = [
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

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut str: String = String::from("");
        let mut num = num;
        let mut i = 0;
        while 0 < num {
            if TABLE[i].0 <= num {
                num -= TABLE[i].0;
                str += TABLE[i].1;
            } else {
                i += 1;
            }
        }
        str
    }
}

// ------

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_12() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
