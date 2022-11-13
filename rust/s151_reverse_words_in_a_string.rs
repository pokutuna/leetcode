impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(" ")
            .filter(|&i| i != "")
            .collect::<Vec<&str>>()
            .iter()
            .rev()
            .fold(String::new(), |sum, i| {
                if sum == "" {
                    i.to_string()
                } else {
                    sum + " " + i
                }
            })
    }
}

// ------

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_151() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".into()),
            "blue is sky the".to_owned()
        );
        assert_eq!(
            Solution::reverse_words("  hello world  ".into()),
            "world hello".to_owned()
        );
        assert_eq!(
            Solution::reverse_words("a good   example".into()),
            "example good a".to_owned(),
        );
    }
}
