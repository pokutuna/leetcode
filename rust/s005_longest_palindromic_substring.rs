// https://leetcode.com/problems/longest-palindromic-substring/

struct Solution {}

impl Solution {
  pub fn longest_palindrome(s: String) -> String {
    let str: Vec<char> = s.chars().collect();

    let (mut longest, mut start, mut end) = (0, 0, 0);
    for i in 0..(str.len() - 1) {
      let i = i as i32;

      for result in vec![Self::scan_lr(&str, i, i), Self::scan_lr(&str, i, i + 1)] {
        match result {
          Some((s, e)) if longest < e - s => {
            longest = e - s;
            start = s;
            end = e;
          }
          _ => (),
        }
      }
    }
    let str = str[(start as usize)..=(end as usize)].iter().collect();
    str
  }

  fn scan_lr(str: &Vec<char>, a: i32, b: i32) -> Option<(i32, i32)> {
    if str[a as usize] != str[b as usize] {
      return None;
    }
    let mut r = 1;
    while 0 <= a - r && b + r < str.len() as i32 && str[(a - r) as usize] == str[(b + r) as usize] {
      r += 1;
    }
    Some((a - r + 1, b + r - 1))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_5() {
    let f = |s| Solution::longest_palindrome(String::from(s));
    assert_eq!(f("babad"), "bab");
    assert_eq!(f("cbbd"), "bb");
    assert_eq!(f("a"), "a");
    assert_eq!(f("ac"), "a");
  }
}
