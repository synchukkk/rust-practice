use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

/// Simple pseudo-random number generator using system time as seed
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        
        SimpleRng {
            state: hasher.finish(),
        }
    }
    
    fn next(&mut self) -> u32 {
        // Linear congruential generator
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.state >> 32) as u32
    }
    
    fn gen_range(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min + 1) as u32;
        let rand_val = self.next() % range;
        min + rand_val as i32
    }
}

/// Generates a random vector of length n with values in range [10..99]
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = SimpleRng::new();
    (0..n).map(|_| rng.gen_range(10, 99)).collect()
}

/// Finds the minimum adjacent sum and returns (sum, first_index, second_index)
fn min_adjacent_sum(data: &[i32]) -> Option<(i32, usize, usize)> {
    if data.len() < 2 {
        return None;
    }
    
    let mut min_sum = data[0] + data[1];
    let mut min_indices = (0, 1);
    
    for i in 1..data.len() - 1 {
        let current_sum = data[i] + data[i + 1];
        if current_sum < min_sum {
            min_sum = current_sum;
            min_indices = (i, i + 1);
        }
    }
    
    Some((min_sum, min_indices.0, min_indices.1))
}

/// Prints the result in a readable format similar to the examples
fn print_result(data: &[i32], result: Option<(i32, usize, usize)>) {
    // Print indexes line
    print!("indexes: ");
    for i in 0..data.len() {
        if i == data.len() - 1 {
            print!("{}.", i);
        } else {
            print!("{}. ", i);
        }
    }
    println!();
    
    // Print data line
    println!("data: {:?}", data);
    
    // Print empty indexes line
    println!("indexes: __ __/");
    
    // Print result
    match result {
        Some((sum, idx1, idx2)) => {
            println!("min adjacent sum={}+{}={} at indexes:{},{}", 
                    data[idx1], data[idx2], sum, idx1, idx2);
        }
        None => {
            println!("No adjacent pair found");
        }
    }
    println!();
}

fn main() {
    // Generate and process multiple random vectors as shown in examples
    for i in 1..=4 {
        println!("Example {}:", i);
        let data = gen_random_vector(20);
        let result = min_adjacent_sum(&data);
        print_result(&data, result);
    }
    
    // Test with the examples from the document
    println!("Testing with provided examples:");
    println!();
    
    let test_data = vec![
        vec![45, 87, 49, 64, 50, 37, 45, 72, 55, 64, 90, 86, 60, 54, 78, 72, 83, 44, 89, 22],
        vec![29, 92, 14, 65, 57, 98, 10, 45, 11, 48, 69, 21, 12, 75, 51, 69, 72, 36, 47, 45],
        vec![19, 86, 66, 95, 40, 24, 90, 74, 98, 37, 26, 44, 76, 86, 48, 63, 11, 38, 29, 40],
        vec![30, 18, 68, 87, 99, 81, 88, 45, 34, 79, 81, 79, 93, 55, 26, 24, 32, 55, 59, 97],
    ];
    
    for (i, data) in test_data.iter().enumerate() {
        println!("Test example {}:", i + 1);
        let result = min_adjacent_sum(data);
        print_result(data, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gen_random_vector() {
        let vec = gen_random_vector(20);
        assert_eq!(vec.len(), 20);
        for &val in &vec {
            assert!(val >= 10 && val <= 99);
        }
    }
    
    #[test]
    fn test_min_adjacent_sum() {
        let data = vec![45, 87, 49, 64, 50, 37, 45, 72, 55, 64];
        let result = min_adjacent_sum(&data);
        assert_eq!(result, Some((82, 5, 6))); // 37 + 45 = 82
    }
    
    #[test]
    fn test_min_adjacent_sum_empty() {
        let data = vec![];
        let result = min_adjacent_sum(&data);
        assert_eq!(result, None);
    }
    
    #[test]
    fn test_min_adjacent_sum_single() {
        let data = vec![42];
        let result = min_adjacent_sum(&data);
        assert_eq!(result, None);
    }
}
