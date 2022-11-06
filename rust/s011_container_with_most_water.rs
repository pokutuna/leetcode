impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp::min;
        let mut since: usize = 0;
        let mut until: usize = height.len() - 1;
        let mut max = 0;

        while since != until {
            let m = min(height[since], height[until]) * (until - since) as i32;
            if m > max {
                max = m;
            }

            if height[since] <= height[until] {
                since += 1;
            } else {
                until -= 1;
            }
        }
        max
    }
}

// ---

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_011() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }

    #[test]
    fn fails_011() {
        assert_eq!(Solution::max_area(vec![2, 3, 4, 5, 18, 17, 6]), 17);
    }
}
