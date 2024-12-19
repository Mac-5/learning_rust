fn main() {

    // write a simple calculator
    // takes user input for two numbers and an operator
    // prints the result of the operation
    // repeat until user types 'exit'
    loop {
        println!("Enter first number: ");
        let mut num1 = String::new();
        std::io::stdin().read_line(&mut num1).unwrap();
        let num1: f64 = num1.trim().parse().unwrap();

        println!("Enter second number: ");
        let mut num2 = String::new();
        std::io::stdin().read_line(&mut num2).unwrap();
        let num2: f64 = num2.trim().parse().unwrap();

        println!("Enter operator: ");
        let mut operator = String::new();
        std::io::stdin().read_line(&mut operator).unwrap();
        let operator = operator.trim();

        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => num1 / num2,
            _ => {
                println!("Invalid operator");
                continue;
            }
        };

        println!("Result: {}", result);

        println!("Type 'exit' to quit, or press enter to continue");
        let mut exit = String::new();
        std::io::stdin().read_line(&mut exit).unwrap();
        if exit.trim() == "exit" {
            break;
        }
    }
}