// hw14.rs - Gray Code Implementation

fn main() {
    // Demonstrate Gray code generation
    println!("Gray Code Examples:");
    for n in 0..=4 {
        let codes = gray(n);
        println!("n={}: {:?}", n, codes);
    }
    
    // Run manual tests
    run_tests();
    println!("All tests passed!");
}

fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()];
    }
    
    let prev = gray(n - 1);
    let mut result = Vec::new();
    
    // First half: add "0" prefix to previous results
    for code in &prev {
        result.push(format!("0{}", code));
    }
    
    // Second half: add "1" prefix to previous results in reverse order
    for code in prev.iter().rev() {
        result.push(format!("1{}", code));
    }
    
    result
}

fn run_tests() {
    let test_data = [
        (0, vec![""]),
        (1, vec!["0", "1"]),
        (2, vec!["00", "01", "10", "11"]),
        (3, vec!["000", "001", "010", "011", "100", "101", "110", "111"]),
        (4, vec!["0000", "0001", "0010", "0011", "0100", "0101", "0110", "0111",
                 "1000", "1001", "1010", "1011", "1100", "1101", "1110", "1111"]),
    ];

    test_data
        .iter()
        .for_each(|(n, out)| {
            let expected: Vec<String> = out.iter().map(|s| s.to_string()).collect();
            assert_eq!(gray(*n), expected);
        });
}

#[test]
fn test() {
    let test_data = [
        (0, vec![""]),
        (1, vec!["0", "1"]),
        (2, vec!["00", "01", "10", "11"]),
        (3, vec!["000", "001", "010", "011", "100", "101", "110", "111"]),
        (4, vec!["0000", "0001", "0010", "0011", "0100", "0101", "0110", "0111",
                 "1000", "1001", "1010", "1011", "1100", "1101", "1110", "1111"]),
    ];

    test_data
        .iter()
        .for_each(|(n, out)| {
            let expected: Vec<String> = out.iter().map(|s| s.to_string()).collect();
            assert_eq!(gray(*n), expected);
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray_code_properties() {
        // Test that adjacent codes differ by exactly one bit
        for n in 1..=4 {
            let codes = gray(n);
            for i in 0..codes.len() - 1 {
                let diff_count = codes[i]
                    .chars()
                    .zip(codes[i + 1].chars())
                    .filter(|(a, b)| a != b)
                    .count();
                assert_eq!(diff_count, 1, "Adjacent codes should differ by exactly one bit");
            }
        }
    }

    #[test]
    fn test_gray_code_length() {
        for n in 0..=5 {
            let codes = gray(n);
            let expected_length = if n == 0 { 1 } else { 2_usize.pow(n as u32) };
            assert_eq!(codes.len(), expected_length);
        }
    }
}
