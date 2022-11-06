impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        match strs.len() {
            0 => return "".to_owned(),
            1 => return strs[0].to_owned(),
            _ => (),
        }

        let chars_vec: Vec<&[u8]> = strs.iter().map(|s| s.as_bytes()).collect();
        let (rep, rest) = (chars_vec[0], &chars_vec[1..]);
        let mut i: usize = 0;
        for c in rep {
            let same = rest.iter().all(|cs| Some(c) == (cs).get(i));
            if same {
                i += 1;
            } else {
                break;
            }
        }

        strs[0][0..i].to_owned()
    }
}

// ------

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(input: &[&str]) -> Vec<String> {
        input.iter().map(|&s| s.into()).collect()
    }

    #[test]
    fn test_14() {
        let solve = Solution::longest_common_prefix;
        assert_eq!(solve(to_vec(&["flower", "flow", "flight"])), "fl");
        assert_eq!(solve(to_vec(&["dog", "racecar", "car"])), "");
    }
}
