struct Solution {}

// https://leetcode.com/problems/longest-substring-without-repeating-characters/

use std::collections::HashMap;

impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let vec: Vec<char> = s.chars().collect();
    let (mut start, mut end, mut answer, size) = (0, 0, 0, s.len());
    let mut map = HashMap::new();

    while start < size && end < size {
      match map.get(&vec[end]) {
        None => {
          map.insert(&vec[end], true);
          end = end + 1;

          let len = end - start;
          if answer < len {
            answer = len;
          }
        }
        Some(_) => {
          map.remove(&vec[start]);
          start = start + 1;
        }
      }
    }
    answer as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_3() {
    assert_eq!(
      Solution::length_of_longest_substring(String::from("abcabcbb")),
      3
    );
    assert_eq!(
      Solution::length_of_longest_substring(String::from("bbbbb")),
      1
    );
    assert_eq!(
      Solution::length_of_longest_substring(String::from("pwwkew")),
      3
    );
    assert_eq!(Solution::length_of_longest_substring(String::from("")), 0);
    assert_eq!(
      Solution::length_of_longest_substring(String::from("abba")),
      2
    );
  }
}
