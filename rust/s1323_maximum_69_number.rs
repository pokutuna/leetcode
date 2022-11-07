impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut buf: Vec<u8> = num.to_string().into_bytes();
        let (six, nine) = ("6".as_bytes()[0], "9".as_bytes()[0]);
        for (i, c) in buf.iter().enumerate() {
            if c == &six {
                buf[i] = nine;
                break;
            }
        }
        String::from_utf8(buf.into())
            .unwrap()
            .parse::<i32>()
            .unwrap()
    }
}

// ------

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1323() {
        assert_eq!(Solution::maximum69_number(9669), 9969);
        assert_eq!(Solution::maximum69_number(9996), 9999);
        assert_eq!(Solution::maximum69_number(9999), 9999);
    }
}
