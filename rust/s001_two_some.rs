use std::collections::HashMap;

struct Solution {}

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, n) in nums.iter().enumerate() {
      map.insert(n, i as i32);
    }
    for (i, n) in nums.iter().enumerate() {
      match map.get(&(target - n)) {
        Some(&m) if m != i as i32 => return vec![i as i32, m],
        _ => (),
      }
    }
    panic!("no pairs to solve it");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
  }
}
