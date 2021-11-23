// https://leetcode.com/problems/reverse-integer/

struct Solution {}

impl Solution {
  pub fn reverse(x: i32) -> i32 {
    let negative = x < 0;
    let mut x: i32 = x.checked_abs().unwrap_or_else(|| return 0);

    let mut vec = Vec::new();
    while x >= 10 {
      vec.push(x % 10);
      x = x / 10;
    }
    vec.push(x);

    let l = vec.len();
    let mut overflowed = false;
    x = 0;
    for i in 0..l {
      x = 10u32
        .checked_pow((l - i - 1) as u32)
        .and_then(|d| (d as i32).checked_mul(vec[i]))
        .and_then(|n| n.checked_add(x))
        .unwrap_or_else(|| {
          overflowed = true;
          0
        });
      if overflowed {
        break;
      }
    }

    if negative {
      x * -1
    } else {
      x
    }
  }
}
// 楽勝かと思ったけど overflow したら 0 を返すという部分のほうが難しい
// checked_* を覚えた
// 入力の段階で abs 取ると overflow するものがあるのはたしかに
// 反転と足し上げを1回でやれば短くなりそう

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_7() {
    let f = |x| Solution::reverse(x);
    assert_eq!(f(123), 321);
    assert_eq!(f(-123), -321);
    assert_eq!(f(120), 21);
    assert_eq!(f(0), 0);
    assert_eq!(f(10), 1);
    // failed
    assert_eq!(f(1534236469), 0);
    assert_eq!(f(-2147483648), 0);
  }
}
