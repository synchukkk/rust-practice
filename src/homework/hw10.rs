fn main() {
    // Demo of the palindrome checker
    let test_numbers = [123, 121, 1221, 0, 7, 12321, 12345];
    
    println!("Palindrome Checker Demo:");
    println!("=======================");
    
    for num in test_numbers {
        let result = is_palindrome(num);
        println!("{} is {} a palindrome", num, if result { "" } else { "not" });
    }
}

fn is_palindrome(x: u32) -> bool {
    // Convert number to string to easily compare digits
    let s = x.to_string();
    let chars: Vec<char> = s.chars().collect();
    
    // Compare characters from both ends moving inward
    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = [
            (123, false),
            (121, true),
            (1221, true),
        ];

        data.iter().for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
        });
    }

    #[test]
    fn additional_tests() {
        // Additional test cases to verify the function works correctly
        assert_eq!(is_palindrome(0), true);      // Single digit
        assert_eq!(is_palindrome(7), true);      // Single digit
        assert_eq!(is_palindrome(10), false);    // Two digits, not palindrome
        assert_eq!(is_palindrome(11), true);     // Two digits, palindrome
        assert_eq!(is_palindrome(12321), true);  // Five digits, palindrome
        assert_eq!(is_palindrome(12345), false); // Five digits, not palindrome
    }
}
