impl Solution {
    #[inline]
    fn large_value() -> i32 {
        // -10^4 <= target <= 10^4
        100000
    }

    fn sum(nums: &[i32], target: i32) -> i32 {
        let (mut sum, h, mut l, mut r) = (Self::large_value(), nums[0], 1, nums.len() - 1);

        while l < r {
            let s = h + nums[l] + nums[r];
            if s == target {
                return target;
            }
            if (s - target).abs() < (sum - target).abs() {
                sum = s;
            }

            if s < target {
                l += 1;
            } else {
                r -= 1;
            }
        }

        sum
    }

    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut result = Self::large_value();
        for i in 0..nums.len() {
            let n = Self::sum(&nums[i..], target);
            if n == target {
                return n;
            }

            if (n - target).abs() < (result - target).abs() {
                result = n;
            }
        }
        result
    }
}

// ------

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0163() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    }

    #[test]
    fn test_163_my() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 10), 2);
        assert_eq!(Solution::three_sum_closest(vec![-4, -1, 1, 2], -10), -4);
    }

    #[test]
    fn test_163_fail() {
        assert_eq!(Solution::three_sum_closest(vec![0, 1, 2], 3), 3);
    }
}
