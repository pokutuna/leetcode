// https://leetcode.com/problems/string-to-integer-atoi/
// trim とか parse 使わずやってみる

struct Solution {}

use std::collections::HashMap;

impl Solution {
  fn overflowed_value(sign: i32) -> i32 {
    if sign < 0 {
      i32::MIN
    } else {
      i32::MAX
    }
  }

  // leetcode 上で HashMap::from するとコケる
  pub fn my_atoi(s: String) -> i32 {
    if s == "" {
      return 0;
    }

    let table = vec![
      ('0', 0),
      ('1', 1),
      ('2', 2),
      ('3', 3),
      ('4', 4),
      ('5', 5),
      ('6', 6),
      ('7', 7),
      ('8', 8),
      ('9', 9),
    ]
    .into_iter()
    .collect::<HashMap<char, i32>>();

    let s: Vec<char> = s.chars().collect();

    let (mut pos, mut sign, mut num, mut overflowed) = (0, 1, 0i32, false);

    // skip leading whitespaces
    for i in pos..s.len() {
      pos = i;
      if s[pos] == ' ' {
        continue;
      } else {
        break;
      }
    }

    // check sign
    // "+-" みたいな入力もくる? その場合どうなるべき? => 0 でよい
    match s[pos] {
      '+' => {
        sign = 1;
        pos += 1;
      }
      '-' => {
        sign = -1;
        pos += 1;
      }
      _ => (),
    }

    // parse digits
    for i in pos..s.len() {
      pos = i;
      if !s[pos].is_ascii_digit() {
        break;
      }
      num = num
        .checked_mul(10)
        .and_then(|n| n.checked_add(table[&s[pos]]))
        .unwrap_or_else(|| {
          overflowed = true;
          0
        });
      if overflowed {
        break;
      }
    }
    if overflowed {
      return Self::overflowed_value(sign);
    }

    num
      .checked_mul(sign)
      .unwrap_or_else(|| Self::overflowed_value(sign))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_8() {
    let f = |s| Solution::my_atoi(s);
    let cases = vec![
      ("42", 42),
      ("   -42", -42),
      ("4193 with words", 4193),
      ("words and 987", 0),
      ("-91283472332", -2147483648),
      // failed
      ("+-12", 0),
      ("", 0),
    ];
    for c in cases.iter() {
      assert_eq!(f(c.0.to_string()), c.1);
    }
  }
}
