// hw05.rs - Greatest Common Divisor implementation

/// Calculates the Greatest Common Divisor of two unsigned 32-bit integers
/// using the Euclidean algorithm
pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    // Демонстрація роботи функції gcd
    println!("GCD examples:");
    println!("gcd(24, 60) = {}", gcd(24, 60));
    println!("gcd(15, 9) = {}", gcd(15, 9)); 
    println!("gcd(37, 11) = {}", gcd(37, 11));
    println!("gcd(120, 80) = {}", gcd(120, 80));
    
    // Запуск тестів програмно (не рекомендується для production)
    println!("\nRunning tests manually:");
    let data = [
        ((24, 60), 12),
        ((15, 9), 3),
        ((15, 6), 3),
        ((140, 40), 20),
        ((24, 16), 8),
        ((100, 10), 10),
        ((120, 80), 40),
        ((80, 120), 40),
        ((100, 20), 20),
        ((37, 11), 1),
        ((120, 90), 30),
    ];

    let mut all_passed = true;
    for ((a, b), expected) in data.iter() {
        let result = gcd(*a, *b);
        if result == *expected {
            println!("✓ gcd({}, {}) = {} (expected {})", a, b, result, expected);
        } else {
            println!("✗ gcd({}, {}) = {} (expected {})", a, b, result, expected);
            all_passed = false;
        }
    }
    
    if all_passed {
        println!("\n🎉 All tests passed!");
    } else {
        println!("\n❌ Some tests failed!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = [
            ((24, 60), 12),
            ((15, 9), 3),
            ((15, 6), 3),
            ((140, 40), 20),
            ((24, 16), 8),
            ((100, 10), 10),
            ((120, 80), 40),
            ((80, 120), 40),
            ((100, 20), 20),
            ((37, 11), 1),
            ((120, 90), 30),
        ];

        for ((a, b), exp) in data.iter() {
            assert_eq!(*exp, gcd(*a, *b));
        }
    }

    #[test]
    fn test_edge_cases() {
        // Test with zero (gcd(a, 0) should equal a)
        assert_eq!(gcd(42, 0), 42);
        assert_eq!(gcd(0, 42), 42);
        
        // Test with same numbers
        assert_eq!(gcd(15, 15), 15);
        
        // Test with 1 (gcd with 1 should always be 1)
        assert_eq!(gcd(1, 100), 1);
        assert_eq!(gcd(100, 1), 1);
    }
}
