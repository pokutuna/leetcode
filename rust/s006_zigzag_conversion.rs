// https://leetcode.com/problems/zigzag-conversion/

struct Solution {}

struct ZigZag {
  i: i32,
  row: i32,
  plus: i32,
}

impl ZigZag {
  fn new(row: &i32) -> ZigZag {
    ZigZag {
      i: 0,
      row: *row,
      plus: 1,
    }
  }
}

impl Iterator for ZigZag {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    if self.row == 1 {
      return Some(0);
    }

    let next = Some(self.i as usize);

    if self.i + self.plus >= self.row {
      self.plus = -1;
    } else if self.i + self.plus < 0 {
      self.plus = 1;
    }
    self.i = self.i + self.plus;

    return next;
  }
}

impl Solution {
  pub fn convert(s: String, num_rows: i32) -> String {
    let s: Vec<char> = s.chars().collect();
    let mut iter = ZigZag::new(&num_rows);

    let mut vec = vec!["".to_string(); num_rows as usize];
    for &c in s.iter() {
      vec[iter.next().unwrap()].push(c);
    }
    vec.join("")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_6() {
    assert_eq!(
      Solution::convert("PAYPALISHIRING".to_string(), 3),
      "PAHNAPLSIIGYIR".to_string()
    );
    assert_eq!(
      Solution::convert("PAYPALISHIRING".to_string(), 4),
      "PINALSIGYAHRPI".to_string()
    );
    assert_eq!(Solution::convert("A".to_string(), 1), "A".to_string());
  }
}
