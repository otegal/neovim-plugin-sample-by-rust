#[derive(Debug, PartialEq)]
struct Calculator;

impl Calculator {
    fn new() -> Self {
        Calculator {}
    }

    fn add(&self, nums: Vec<i64>) -> i64 {
        nums.iter().sum()
    }

    fn sub(&self, nums: Vec<i64>) -> i64 {
        nums.iter().sum::<i64>() * -1
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_new() {
        assert_eq!(Calculator {}, Calculator::new())
    }

    #[test]
    fn test_calculator_add() {
        let cal = Calculator::new();
        assert_eq!(10, cal.add(vec!(1, 2, 3, 4)));
        assert_eq!(-10, cal.add(vec!(-1, -2, -3, -4)));
    }

    #[test]
    fn test_calculator_sub() {
        let cal = Calculator::new();
        assert_eq!(-10, cal.sub(vec!(1, 2, 3, 4)));
        assert_eq!(10, cal.sub(vec!(-1, -2, -3, -4)));
    }
}
