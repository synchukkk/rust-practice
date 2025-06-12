// hw13.rs

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    a: Point, // top-left
    b: Point, // bottom-right
}

impl Rectangle {
    fn area(&self) -> i32 {
        (self.b.x - self.a.x) * (self.a.y - self.b.y)
    }
    
    // Check if this rectangle intersects with another
    fn intersects(&self, other: &Rectangle) -> bool {
        !(self.b.x <= other.a.x || other.b.x <= self.a.x ||
          self.b.y >= other.a.y || other.b.y >= self.a.y)
    }
    
    // Get intersection rectangle if they intersect
    fn intersection(&self, other: &Rectangle) -> Option<Rectangle> {
        if !self.intersects(other) {
            return None;
        }
        
        let left = self.a.x.max(other.a.x);
        let right = self.b.x.min(other.b.x);
        let top = self.a.y.min(other.a.y);
        let bottom = self.b.y.max(other.b.y);
        
        Some(Rectangle {
            a: Point { x: left, y: top },
            b: Point { x: right, y: bottom },
        })
    }
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    if xs.is_empty() {
        return 0;
    }
    
    // Use the inclusion-exclusion principle
    // Total area = Sum of individual areas 
    //            - Sum of pairwise intersections
    //            + Sum of triple intersections
    //            - Sum of quadruple intersections
    //            + ...
    
    let n = xs.len();
    let mut total_area = 0;
    
    // Generate all possible subsets of rectangles
    for mask in 1..(1 << n) {
        let mut current_rectangles = Vec::new();
        let mut bit_count = 0;
        
        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                current_rectangles.push(&xs[i]);
                bit_count += 1;
            }
        }
        
        // Find intersection of all rectangles in current subset
        if let Some(intersection_area) = calculate_intersection_area(&current_rectangles) {
            // Apply inclusion-exclusion principle
            if bit_count % 2 == 1 {
                total_area += intersection_area;
            } else {
                total_area -= intersection_area;
            }
        }
    }
    
    total_area
}

fn calculate_intersection_area(rectangles: &[&Rectangle]) -> Option<i32> {
    if rectangles.is_empty() {
        return None;
    }
    
    if rectangles.len() == 1 {
        return Some(rectangles[0].area());
    }
    
    // Find the intersection of all rectangles
    let mut result = rectangles[0].clone();
    
    for &rect in &rectangles[1..] {
        if let Some(intersection) = result.intersection(rect) {
            result = intersection;
        } else {
            return None; // No intersection
        }
    }
    
    Some(result.area())
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_occupied_test() {
        let data = test_data();
        let occupied = area_occupied(&data);
        assert_eq!(occupied, 60);
    }
    
    #[test]
    fn test_individual_areas() {
        let data = test_data();
        
        // Red rectangle: width=3, height=6, area=18
        assert_eq!(data[0].area(), 18);
        
        // Green rectangle: width=10, height=2, area=20  
        assert_eq!(data[1].area(), 20);
        
        // Blue rectangle: width=4, height=8, area=32
        assert_eq!(data[2].area(), 32);
        
        // Total individual areas: 18 + 20 + 32 = 70
        let total_individual: i32 = data.iter().map(|r| r.area()).sum();
        assert_eq!(total_individual, 70);
    }
    
    #[test]
    fn test_rectangle_intersection() {
        let data = test_data();
        
        // Test intersection between red and green rectangles
        let red_green_intersection = data[0].intersection(&data[1]);
        assert!(red_green_intersection.is_some());
        
        if let Some(intersection) = red_green_intersection {
            assert_eq!(intersection.area(), 6); // 2x3 overlap
        }
    }
    
    #[test]
    fn test_empty_input() {
        let empty_data = Vec::new();
        assert_eq!(area_occupied(&empty_data), 0);
    }
    
    #[test]
    fn test_single_rectangle() {
        let single_rect = vec![Rectangle {
            a: Point { x: 0, y: 5 },
            b: Point { x: 3, y: 2 },
        }];
        assert_eq!(area_occupied(&single_rect), 9); // 3x3 = 9
    }
}

fn main() {
    let data = test_data();
    let occupied = area_occupied(&data);
    
    println!("Individual rectangle areas:");
    for (i, rect) in data.iter().enumerate() {
        println!("Rectangle {}: {}", i + 1, rect.area());
    }
    
    println!("Total individual areas: {}", data.iter().map(|r| r.area()).sum::<i32>());
    println!("Actual occupied area: {}", occupied);
}
