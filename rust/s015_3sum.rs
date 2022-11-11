use std::collections::HashSet;

impl Solution {
    /// nums expects sorted
    fn two_sum(nums: &[i32], result: &mut Vec<Vec<i32>>) {
        if nums.len() < 3 {
            return;
        }

        let n = nums[0];
        let (mut l, mut r) = (1, nums.len() - 1);

        while l < r {
            let sum = n + nums[l] + nums[r];
            if sum == 0 {
                let v = vec![n, nums[l], nums[r]];
                result.push(v);
                l += 1;
            } else if sum < 0 {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut result: Vec<Vec<i32>> = vec![];
        for i in 0..nums.len() {
            Self::two_sum(&nums[i..], &mut result);
        }

        result
            .iter()
            .map(|v| (v[0], v[1], v[2]))
            .collect::<HashSet<(i32, i32, i32)>>()
            .iter()
            .map(|t| vec![t.0, t.1, t.2])
            .collect()
    }
}

// ------

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(col: Vec<Vec<i32>>) -> HashSet<(i32, i32, i32)> {
        col.iter().map(|v| (v[0], v[1], v[2])).collect()
    }

    #[test]
    fn test_015() {
        assert_eq!(
            set(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])),
            set(vec![vec![-1, -1, 2], vec![-1, 0, 1]])
        );

        assert_eq!(
            set(Solution::three_sum(vec![0, 1, 1])),
            set(vec![] as Vec<Vec<i32>>)
        );

        assert_eq!(
            set(Solution::three_sum(vec![0, 0, 0])),
            set(vec![vec![0, 0, 0]])
        );
    }

    #[test]
    fn fails() {
        assert_eq!(
            set(Solution::three_sum(vec![
                -1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4
            ])),
            set(vec![
                vec![-4, 0, 4],
                vec![-4, 1, 3],
                vec![-3, -1, 4],
                vec![-3, 0, 3],
                vec![-3, 1, 2],
                vec![-2, -1, 3],
                vec![-2, 0, 2],
                vec![-1, -1, 2],
                vec![-1, 0, 1]
            ])
        );
    }
}
