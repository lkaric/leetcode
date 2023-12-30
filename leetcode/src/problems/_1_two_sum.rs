use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm = HashMap::with_capacity(nums.len());

        for (i, &num) in nums.iter().enumerate() {
            match hm.get(&num) {
                Some(&j) => return vec![i as i32, j as i32],
                None => {
                    hm.insert(target - num, i);
                }
            }
        }

        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::_1_two_sum::Solution;

    #[test]
    fn case_9() {
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![1, 0]);
    }

    #[test]
    fn case_6() {
        let result = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![2, 1]);
    }

    #[test]
    fn case_4() {
        let result = Solution::two_sum(vec![3, 1], 4);
        assert_eq!(result, vec![1, 0]);
    }
}
