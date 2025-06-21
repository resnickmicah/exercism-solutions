#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];
    for next_input in inputs {
        match next_input {
            CalculatorInput::Value(number) => stack.push(*number),
            _ => {
                if stack.len() < 2 {
                    return None;
                }
                let (first, second) = (stack.pop().unwrap(), stack.pop().unwrap());

                match next_input {
                    CalculatorInput::Add => stack.push(first + second),
                    CalculatorInput::Subtract => stack.push(second - first),
                    CalculatorInput::Multiply => stack.push(first * second),
                    CalculatorInput::Divide => stack.push(second / first),
                    _ => return None,
                }
            }
        }
    }
    if stack.len() > 1 {
        return None;
    }
    stack.pop()
}
