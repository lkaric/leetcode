pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string() == x.to_string().chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::_9_palindrome_number::Solution;

    #[test]
    fn case_121() {
        let result = Solution::is_palindrome(121);
        assert_eq!(result, true);
    }

    #[test]
    fn case_negative_121() {
        let result = Solution::is_palindrome(-121);
        assert_eq!(result, false);
    }

    #[test]
    fn case_10() {
        let result = Solution::is_palindrome(10);
        assert_eq!(result, false);
    }
}
