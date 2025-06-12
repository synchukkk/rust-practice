fn rotate(s: String, n: isize) -> String {
    if s.is_empty() {
        return s;
    }
    
    let len = s.len() as isize;
    
    // Нормалізуємо зсув, щоб він був у межах довжини рядка
    let shift = ((n % len) + len) % len;
    
    if shift == 0 {
        return s;
    }
    
    // Конвертуємо в байти для роботи з індексами
    let bytes = s.as_bytes();
    let split_point = len - shift;
    
    // Розділяємо рядок і переставляємо частини
    let first_part = &bytes[split_point as usize..];
    let second_part = &bytes[..split_point as usize];
    
    // Об'єднуємо частини
    let mut result = Vec::with_capacity(bytes.len());
    result.extend_from_slice(first_part);
    result.extend_from_slice(second_part);
    
    // Конвертуємо назад у String
    String::from_utf8(result).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0, "abcdefgh"),
            (8, "abcdefgh"),
            (-8, "abcdefgh"),
            (1, "habcdefg"),
            (2, "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10, "cdefghab"),
        ];

        shifts
            .iter()
            .for_each(|(n, exp)| {
                assert_eq!(
                    rotate(s.to_string(), *n),
                    exp.to_string()
                )
            });
    }
}

fn main() {
    // Приклад використання
    let test_string = "abcdefgh".to_string();
    
    println!("Оригінал: {}", test_string);
    println!("Зсув на 1: {}", rotate(test_string.clone(), 1));
    println!("Зсув на -1: {}", rotate(test_string.clone(), -1));
    println!("Зсув на 2: {}", rotate(test_string.clone(), 2));
}
