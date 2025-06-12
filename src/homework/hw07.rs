fn main() {
    // Example usage
    let test_string = "Hello World!".to_string();
    let inverted = invert_the_case(test_string.clone());
    println!("Original: {}", test_string);
    println!("Inverted: {}", inverted);
    
    let cyrillic_test = "Привет Мир!".to_string();
    let inverted_cyrillic = invert_the_case(cyrillic_test.clone());
    println!("Original: {}", cyrillic_test);
    println!("Inverted: {}", inverted_cyrillic);
}

fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}

#[test]
fn test() {
    let data = [
        ("Hello", "hELLO"),
        ("Привет", "пРИВЕТ"),
    ];

    data.iter().for_each(|(a, b)| {
        assert_eq!(
            invert_the_case(a.to_string()),
            b.to_string()
        );
        assert_eq!(
            invert_the_case(b.to_string()),
            a.to_string()
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_additional_cases() {
        // Test with mixed case
        assert_eq!(invert_the_case("RuSt".to_string()), "rUsT");
        
        // Test with numbers and special characters
        assert_eq!(invert_the_case("Hello123!".to_string()), "hELLO123!");
        
        // Test with empty string
        assert_eq!(invert_the_case("".to_string()), "");
        
        // Test with only uppercase
        assert_eq!(invert_the_case("RUST".to_string()), "rust");
        
        // Test with only lowercase
        assert_eq!(invert_the_case("rust".to_string()), "RUST");
    }
}
