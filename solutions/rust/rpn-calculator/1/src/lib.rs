#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut input_stack: Vec<CalculatorInput> = vec![];
    for next_input in inputs {
        let result = match next_input {
            CalculatorInput::Add => handle_add(&mut input_stack),
            CalculatorInput::Subtract => handle_subtract(&mut input_stack),
            CalculatorInput::Multiply => handle_multiply(&mut input_stack),
            CalculatorInput::Divide => handle_divide(&mut input_stack),
            CalculatorInput::Value(n) =>  Some(CalculatorInput::Value(*n)),
        };
        if let Some(CalculatorInput::Value(n)) = result {
            input_stack.push(CalculatorInput::Value(n));
        } else {
            return None;
        }
    }
    if input_stack.len() > 1 {
        return None;
    }
    if let Some(CalculatorInput::Value(n)) = input_stack.pop() {
        Some(n)
    } else {
        None
    }
}

fn handle_add(input_stack: &mut Vec<CalculatorInput>) -> Option<CalculatorInput> {
    if let (Some(CalculatorInput::Value(first)), Some(CalculatorInput::Value(second))) = (input_stack.pop(), input_stack.pop()) {
        Some(CalculatorInput::Value(first + second))
    } else {
        None
    }
}

fn handle_subtract(input_stack: &mut Vec<CalculatorInput>) -> Option<CalculatorInput> {
    if let (Some(CalculatorInput::Value(first)), Some(CalculatorInput::Value(second))) = (input_stack.pop(), input_stack.pop()) {
        Some(CalculatorInput::Value(second - first))
    } else {
        None
    }
}

fn handle_multiply(input_stack: &mut Vec<CalculatorInput>) -> Option<CalculatorInput> {
    if let (Some(CalculatorInput::Value(first)), Some(CalculatorInput::Value(second))) = (input_stack.pop(), input_stack.pop()) {
        Some(CalculatorInput::Value(first * second))
    } else {
        None
    }
}

fn handle_divide(input_stack: &mut Vec<CalculatorInput>) -> Option<CalculatorInput> {
    if let (Some(CalculatorInput::Value(first)), Some(CalculatorInput::Value(second))) = (input_stack.pop(), input_stack.pop()) {
        Some(CalculatorInput::Value(second / first))
    } else {
        None
    }
}