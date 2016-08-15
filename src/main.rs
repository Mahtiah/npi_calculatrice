use std::io::Write;

fn read_string() -> String
{
    let mut operation = String::new();
    std::io::stdin().read_line(&mut operation).unwrap();
    operation
}

fn split_into_whitespaces(content : &str) -> Vec<&str>
{
    let result : Vec<_> = content.split_whitespace().collect();
    result
}

fn apply_operator(lhs : f64, rhs : f64, token : &str) -> Option<f64>
{
    match token
    {
        "+" => Some(lhs + rhs),
        "-" => Some(lhs - rhs),
        "*" => Some(lhs * rhs),
        "/" => Some(lhs / rhs),
        "%" => Some(lhs % rhs),
        _ => None
    }
}

fn compute(tokens : &[&str]) -> f64
{
    let mut operation_stack = Vec::new();

    for token in tokens
    {
        if let Ok(num) = token.parse::<f64>()
        {
            operation_stack.push(num);
        }
        else
        {
            let rhs = operation_stack.pop().unwrap();
            let lhs = operation_stack.pop().unwrap();

            if let Some(result) = apply_operator(lhs, rhs, token)
            {
                operation_stack.push(result);
            }
            else
            {
                panic!("This token is neither a number nor an operator : {}", token);
            }
        }
    }

    operation_stack.pop().unwrap()
}

fn main()
{
    print!("Enter an operation : ");
    std::io::stdout().flush();
    println!("Result : {}", compute(&split_into_whitespaces(&read_string())));
}
