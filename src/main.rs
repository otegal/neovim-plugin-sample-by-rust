use neovim_lib::{Neovim, NeovimApi, Session};

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

struct EventHandler {
    nvim: Neovim,
    calculator: Calculator,
}

impl EventHandler {
    fn new() -> EventHandler {
        let session = Session::new_parent().unwrap(); // In case of Err, want to panic
        let nvim = Neovim::new(session);
        let calculator = Calculator::new();

        EventHandler { nvim, calculator }
    }

    fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, values) in receiver {
            match Messages::from(event) {
                Messages::Add => {
                    let nums = values
                        .iter()
                        .map(|v| v.as_i64().unwrap())
                        .collect::<Vec<i64>>();
                    let sum = self.calculator.add(nums);
                    self.nvim
                        .command(&format!("echo \"Sum: {}\"", sum.to_string()))
                        .unwrap();
                }
                Messages::Sub => {
                    let nums = values
                        .iter()
                        .map(|v| v.as_i64().unwrap())
                        .collect::<Vec<i64>>();
                    let sub_sum = self.calculator.sub(nums);
                    self.nvim
                        .command(&format!("echo \"Sum (Sub): {}\"", sub_sum.to_string()))
                        .unwrap();
                }
                Messages::Unknown(event) => {
                    self.nvim
                        .command(&format!("echo \"Unknown command: {}\"", event))
                        .unwrap();
                }
            }
        }
    }
}

enum Messages {
    Add,
    Sub,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "add" => Messages::Add,
            "sub" => Messages::Sub,
            _ => Messages::Unknown(event),
        }
    }
}

fn main() {
    let mut event_handler = EventHandler::new();
    event_handler.recv();
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
