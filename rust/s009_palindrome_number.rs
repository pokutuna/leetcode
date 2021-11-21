// https://leetcode.com/problems/palindrome-number/

struct Solution {}

impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
      false
    } else {
      let str = x.to_string();
      str == str.chars().rev().collect::<String>()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_9() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(-121), false);
    assert_eq!(Solution::is_palindrome(10), false);
    assert_eq!(Solution::is_palindrome(-101), false);
  }
}
