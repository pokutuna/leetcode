// https://leetcode.com/problems/median-of-two-sorted-arrays/

struct Solution {}

impl Solution {
  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (l1, l2) = (nums1.len(), nums2.len());
    let (center, odd) = ((l1 + l2) / 2, (l1 + l2) % 2 == 1);

    let (mut i1, mut i2, mut buf1, mut buf2) = (0, 0, 0, 0);
    while i1 + i2 < center + 1 {
      buf1 = buf2;
      let left = match (nums1.get(i1), nums2.get(i2)) {
        (Some(&a), Some(&b)) if a < b => true,
        (Some(&a), Some(&b)) if a >= b => false,
        (Some(_), None) => true,
        (None, Some(_)) => false,
        _ => panic!(),
      };
      if left {
        buf2 = nums1[i1];
        i1 += 1;
      } else {
        buf2 = nums2[i2];
        i2 += 1;
      }
    }

    if odd {
      buf2 as f64
    } else {
      (buf1 + buf2) as f64 / 2.0
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_6() {
    let f = |a, b| Solution::find_median_sorted_arrays(a, b);
    assert_eq!(f(vec![1, 3], vec![2]), 2.00);
    assert_eq!(f(vec![1, 2], vec![3, 4]), 2.50);
    assert_eq!(f(vec![0, 0], vec![0, 0]), 0.00);
    assert_eq!(f(vec![], vec![1]), 1.00);
    assert_eq!(f(vec![2], vec![]), 2.00);
  }
}
