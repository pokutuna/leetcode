struct MedianFinder {
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        MedianFinder { nums: vec![] }
    }

    fn add_num(&mut self, num: i32) {
        self.nums.push(num)
    }

    fn find_median(&mut self) -> f64 {
        self.nums.sort();
        let (m, d) = (self.nums.len() % 2, self.nums.len() / 2);
        if m == 0 {
            (self.nums[d - 1] + self.nums[d]) as f64 / 2 as f64
        } else {
            self.nums[d] as f64
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

// ------

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    enum Op {
        MedianFinder,
        AddNum(i32),
        FindMedian,
    }

    struct Executor {
        m: MedianFinder,
    }

    impl Executor {
        fn new() -> Self {
            Executor {
                m: MedianFinder::new(),
            }
        }

        fn run(&mut self, op: &Op) -> Option<f64> {
            match op {
                Op::MedianFinder => {
                    self.m = MedianFinder::new();
                    None
                }
                Op::AddNum(n) => {
                    self.m.add_num(*n);
                    None
                }
                Op::FindMedian => Some(self.m.find_median()),
            }
        }
    }

    #[test]
    fn test_295() {
        use Op::*;
        let ops = vec![
            (MedianFinder, None),
            (AddNum(1), None),
            (AddNum(2), None),
            (FindMedian, Some(1.5)),
            (AddNum(3), None),
            (FindMedian, Some(2.0)),
        ];
        let mut e = Executor::new();
        for (o, expect) in ops.iter() {
            println!("{:?}", o);
            assert_eq!(e.run(o), *expect);
        }
    }

    #[test]
    fn test_295_my() {
        use Op::*;
        let ops = vec![
            (MedianFinder, None),
            (AddNum(1), None),
            (AddNum(1), None),
            (FindMedian, Some(1.0)),
            (AddNum(1), None),
            (FindMedian, Some(1.0)),
            //
            (MedianFinder, None),
            (AddNum(3), None),
            (FindMedian, Some(3.0)),
            (AddNum(1), None),
            (FindMedian, Some(2.0)),
            (AddNum(10), None),
            (FindMedian, Some(3.0)),
            (AddNum(2), None),
            (FindMedian, Some(2.5)),
        ];
        let mut e = Executor::new();
        for (o, expect) in ops.iter() {
            println!("{:?}", o);
            assert_eq!(e.run(o), *expect);
        }
    }
}
