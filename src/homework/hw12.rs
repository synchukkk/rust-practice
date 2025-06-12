// hw12.rs - Shipments Problem Solution

// Note: For rand functionality, add to Cargo.toml: rand = "0.8"
// use rand::Rng;

/// Calculates the minimum number of moves to equalize cargo across all ships
/// Returns Some(moves) if possible, None if impossible
fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    if shipments.is_empty() {
        return Some(0);
    }
    
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;
    
    // Check if equal distribution is possible
    if total % n != 0 {
        return None;
    }
    
    let target = total / n;
    
    // Calculate total excess cargo (which equals total deficit)
    let total_moves: u32 = shipments
        .iter()
        .filter(|&&weight| weight > target)
        .map(|&weight| weight - target)
        .sum();
    
    Some(total_moves as usize)
}

/// Alternative signature for when equal distribution might not be possible
fn count_permutation_result(shipments: &Vec<u32>) -> Result<usize, String> {
    if shipments.is_empty() {
        return Ok(0);
    }
    
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;
    
    if total % n != 0 {
        return Err(format!(
            "Cannot distribute {} units equally among {} ships", 
            total, n
        ));
    }
    
    let target = total / n;
    let total_moves: u32 = shipments
        .iter()
        .filter(|&&weight| weight > target)
        .map(|&weight| weight - target)
        .sum();
    
    Ok(total_moves as usize)
}

/// Generates a vector of shipments that can be distributed equally
/// Simple implementation without external dependencies
fn gen_shipments(n: usize) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }
    
    // Simple deterministic generation for demonstration
    // In real implementation, you would use rand crate
    let target = 5; // Base weight
    let mut shipments = vec![target; n];
    
    // Create some variation that sums to 0
    let variations = match n {
        1 => vec![0],
        2 => vec![2, -2],
        3 => vec![3, -1, -2],
        4 => vec![4, -2, -1, -1],
        5 => vec![3, -2, 1, -1, -1],
        _ => {
            // For larger n, create a pattern
            let mut vars = vec![0; n];
            for i in 0..n/2 {
                vars[i] = (i as i32 + 1);
                if i + n/2 < n {
                    vars[i + n/2] = -(i as i32 + 1);
                }
            }
            vars
        }
    };
    
    // Apply variations
    for (i, &var) in variations.iter().enumerate() {
        if i < shipments.len() {
            shipments[i] = (shipments[i] as i32 + var).max(1) as u32;
        }
    }
    
    // Ensure total is divisible by n by adjusting last element
    let current_total: u32 = shipments.iter().sum();
    let target_total = (current_total / n as u32) * n as u32;
    if current_total != target_total {
        let diff = current_total as i32 - target_total as i32;
        if shipments[n-1] as i32 - diff > 0 {
            shipments[n-1] = (shipments[n-1] as i32 - diff) as u32;
        }
    }
    
    shipments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let shipments = vec![8, 2, 2, 4, 4];
        assert_eq!(count_permutation(&shipments), Some(4));
    }

    #[test]
    fn test_example_2() {
        let shipments = vec![9, 3, 7, 2, 9];
        assert_eq!(count_permutation(&shipments), Some(7));
    }

    #[test]
    fn test_impossible_case() {
        let shipments = vec![1, 2, 3]; // sum = 6, can't divide by 3 equally
        assert_eq!(count_permutation(&shipments), None);
    }

    #[test]
    fn test_already_equal() {
        let shipments = vec![5, 5, 5, 5];
        assert_eq!(count_permutation(&shipments), Some(0));
    }

    #[test]
    fn test_empty() {
        let shipments = vec![];
        assert_eq!(count_permutation(&shipments), Some(0));
    }

    #[test]
    fn test_gen_shipments() {
        let shipments = gen_shipments(5);
        assert_eq!(shipments.len(), 5);
        
        // Should be possible to equalize
        assert!(count_permutation(&shipments).is_some());
    }
}

fn main() {
    println!("Shipments Problem Solution");
    println!("==========================");
    
    // Test with provided examples
    let example1 = vec![8, 2, 2, 4, 4];
    println!("Example 1: {:?}", example1);
    match count_permutation(&example1) {
        Some(moves) => println!("Minimum moves: {}", moves),
        None => println!("Impossible to equalize"),
    }
    println!();
    
    let example2 = vec![9, 3, 7, 2, 9];
    println!("Example 2: {:?}", example2);
    match count_permutation(&example2) {
        Some(moves) => println!("Minimum moves: {}", moves),
        None => println!("Impossible to equalize"),
    }
    println!();
    
    // Test impossible case
    let impossible = vec![1, 2, 3];
    println!("Impossible case: {:?}", impossible);
    match count_permutation(&impossible) {
        Some(moves) => println!("Minimum moves: {}", moves),
        None => println!("Impossible to equalize"),
    }
    println!();
    
    // Generate and test random shipments
    println!("Generated shipments:");
    for i in 1..=3 {
        let generated = gen_shipments(5);
        println!("Generated {}: {:?}", i, generated);
        match count_permutation(&generated) {
            Some(moves) => println!("Minimum moves: {}", moves),
            None => println!("Impossible to equalize"),
        }
        println!();
    }
}

/*
EXPLANATION:

1. **Algorithm Logic:**
   - Calculate the total cargo and check if it can be divided equally among all ships
   - If total % n != 0, it's impossible to equalize
   - If possible, calculate the target weight per ship (total / n)
   - Sum up all excess cargo from ships above target - this equals the minimum moves needed

2. **Why this works:**
   - Each move transfers 1 ton from one ship to another
   - The total excess cargo must be moved to ships with deficit
   - Since we're looking for minimum moves, we directly calculate total excess

3. **Examples:**
   - [8, 2, 2, 4, 4]: total=20, target=4, excess: (8-4)+(4-4)+(4-4) = 4 moves
   - [9, 3, 7, 2, 9]: total=30, target=6, excess: (9-6)+(7-6)+(9-6) = 3+1+3 = 7 moves

4. **Function Signatures:**
   - `count_permutation() -> Option<usize>`: Returns None if impossible
   - `count_permutation_result() -> Result<usize, String>`: Returns Error with message if impossible

5. **Generation Strategy:**
   - Uses deterministic patterns to create valid shipment distributions
   - For random generation, add `rand = "0.8"` to Cargo.toml and uncomment rand import
   
6. **To use with Cargo.toml:**
   Add this to your Cargo.toml file:
   ```toml
   [dependencies]
   rand = "0.8"
   ```
   Then uncomment the rand import and replace gen_shipments with random version.
*/
