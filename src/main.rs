use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first = args.nth(1).unwrap();
  let operator = args.nth(0).unwrap().chars().next().unwrap();
  let second = args.nth(0).unwrap();

  let first_number = first.parse::<f32>().unwrap();
  let second_number = second.parse::<f32>().unwrap();

  let operation = operate(operator, first_number, second_number);

  let result = output(first_number, operator, second_number, operation);
  println!("{:?}", result);
}

fn operate(operator: char, operand1: f32, operand2: f32) -> f32 {
  match operator {
    '+' => operand1 + operand2,
    '-' => operand1 - operand2,
    '*' | 'x' | 'X' => operand1 * operand2,
    '/' => operand1 / operand2,
    _ => {
      panic!("Invalid operator")
    }
  }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}
