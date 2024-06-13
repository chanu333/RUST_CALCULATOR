use clap::{Arg, Command};



mod operations;
use operations::{add, sub, mul, div};

fn main() {
    let matches = Command::new("cli-calc")
        .version("1.0")
        .author("CHANAKYA")
        .about("A CLI calculator")
        .arg(
            Arg::new("operation")
                .short('o')
                .long("operation")
                .value_name("OPERATION")
                .help("Sets the operation to perform")
                .required(true)
                .value_parser(["add", "sub", "mul", "div", "+", "-", "*", "/"]),
        )
        .arg(
            Arg::new("operand1")
                .short('a')
                .long("operand1")
                .value_name("OPERAND1")
                .help("Sets the first operand")
                .required(true),
        )
        .arg(
            Arg::new("operand2")
                .short('b')
                .long("operand2")
                .value_name("OPERAND2")
                .help("Sets the second operand")
                .required(true),
        )
        .get_matches();


    let operation = matches.get_one::<String>("operation").unwrap().as_str();
    let operand1: f64 = matches.get_one::<String>("operand1").unwrap().parse().expect("Invalid operand1");
    let operand2: f64 = matches.get_one::<String>("operand2").unwrap().parse().expect("Invalid operand2");

    let result = match operation {
        "add" | "+" => add(operand1, operand2),
        "sub" | "-" => sub(operand1, operand2),
        "mul" | "*" => mul(operand1, operand2),
        "div" | "/" => match div(operand1, operand2) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        },
        _ => {
            eprintln!("Invalid operation");
            return;
        }
    };

    println!("The result of {} {} {} is: {}", operand1, operation, operand2, result);
}
