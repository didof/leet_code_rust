// https://rustgym.com/leetcode/9

struct PalindromeNumber;

impl PalindromeNumber {
    fn is_palindrome(mut x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut digits: Vec<i32> = Vec::with_capacity(10);
        while x != 0 {
            digits.push(x % 10);
            x /= 10;
        }

        for i in 0..(digits.len() / 2) {
            if digits[i] != digits[digits.len() - i - 1] {
                return false;
            }
        }

        true
    }
}

#[test]
fn test() {
    assert_eq!(PalindromeNumber::is_palindrome(121), true);
    assert_eq!(PalindromeNumber::is_palindrome(123), false);
    assert_eq!(PalindromeNumber::is_palindrome(-121), false);
}
