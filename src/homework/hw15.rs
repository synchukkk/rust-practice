// hw15.rs - Криптоарифметична задача
// Задача: muxa × a = slon
// Потрібно знайти значення для кожної літери (0-9), де всі літери мають різні значення

fn solve_cryptoarithmetic() -> Vec<(u32, u32, u32, u32, u32, u32, u32, u32)> {
    let mut solutions = Vec::new();
    
    // Перебираємо всі можливі значення для кожної змінної
    for m in 1..=9 {  // m не може бути 0, оскільки це перша цифра числа muxa
        for u in 0..=9 {
            if u == m { continue; }
            for x in 0..=9 {
                if x == m || x == u { continue; }
                for a in 1..=9 {  // a не може бути 0, оскільки це множник
                    if a == m || a == u || a == x { continue; }
                    for s in 1..=9 {  // s не може бути 0, оскільки це перша цифра числа slon
                        if s == m || s == u || s == x || s == a { continue; }
                        for l in 0..=9 {
                            if l == m || l == u || l == x || l == a || l == s { continue; }
                            for o in 0..=9 {
                                if o == m || o == u || o == x || o == a || o == s || o == l { continue; }
                                for n in 0..=9 {
                                    if n == m || n == u || n == x || n == a || n == s || n == l || n == o { continue; }
                                    
                                    // Обчислюємо числа
                                    let muxa = m * 1000 + u * 100 + x * 10 + a;
                                    let slon = s * 1000 + l * 100 + o * 10 + n;
                                    
                                    // Перевіряємо умову: muxa × a = slon
                                    if muxa * a == slon {
                                        solutions.push((m, u, x, a, s, l, o, n));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    solutions
}

fn print_solution(m: u32, u: u32, x: u32, a: u32, s: u32, l: u32, o: u32, n: u32) {
    println!("  {}{}{}{}", m, u, x, a);
    println!("x    {}", a);
    println!("  ------");
    println!("  {}{}{}{}", s, l, o, n);
    println!();
    println!("Перевірка: {} × {} = {}", 
             m * 1000 + u * 100 + x * 10 + a, 
             a, 
             s * 1000 + l * 100 + o * 10 + n);
    println!();
}

fn main() {
    println!("Розв'язання криптоарифметичної задачі: muxa × a = slon");
    println!("Де кожна літера представляє унікальну цифру (0-9)");
    println!();
    
    let solutions = solve_cryptoarithmetic();
    
    println!("Знайдено {} рішень:", solutions.len());
    println!();
    
    for (i, (m, u, x, a, s, l, o, n)) in solutions.iter().enumerate() {
        println!("Рішення {}:", i + 1);
        print_solution(*m, *u, *x, *a, *s, *l, *o, *n);
        println!("Змінні: m={}, u={}, x={}, a={}, s={}, l={}, o={}, n={}", 
                 m, u, x, a, s, l, o, n);
        println!("{}", "-".repeat(50));
    }
    
    if solutions.is_empty() {
        println!("Рішень не знайдено!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_solution_validity() {
        let solutions = solve_cryptoarithmetic();
        
        for (m, u, x, a, s, l, o, n) in solutions {
            let muxa = m * 1000 + u * 100 + x * 10 + a;
            let slon = s * 1000 + l * 100 + o * 10 + n;
            
            // Перевіряємо основну умову
            assert_eq!(muxa * a, slon);
            
            // Перевіряємо, що всі цифри різні
            let digits = vec![m, u, x, a, s, l, o, n];
            let mut unique_digits = digits.clone();
            unique_digits.sort();
            unique_digits.dedup();
            assert_eq!(digits.len(), unique_digits.len());
            
            // Перевіряємо, що перші цифри не нулі
            assert_ne!(m, 0);
            assert_ne!(s, 0);
            assert_ne!(a, 0);
        }
    }
}
