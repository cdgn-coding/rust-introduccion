use regex::Regex;
use std::io::{stdin};

fn main() {
    println!("Bienvenido o bienvenida.");
    println!("Introduzca una expresion matematica o pida ayuda con 'help'.");
    loop {
        // Regex definition
        let re_math = Regex::new(r"(\d+)\s?(\+|-|\*|/)\s?(\d+)").unwrap();
        let re_commands = Regex::new(r"(quit|help)").unwrap();

        // Operation Capture
        let mut exp = String::new();
        stdin().read_line(&mut exp).unwrap();
        let exp = exp.trim();

        // Regex Commands
        let commands_caps = re_commands.captures(exp);
        if commands_caps.is_some() {
            let command = commands_caps.unwrap().get(1).unwrap().as_str();
            match command {
                "quit" => break,
                "help" => println!("Introduzca una expresión matemática"),
                _ => println!("Comando no reconocido"),
            }
            continue
        }

        // Regex Match
        let operation_caps = re_math.captures(exp);
        if operation_caps.is_some() {
            let caps = operation_caps.unwrap();
            let elem1: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let elem2: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
            let op: &str = caps.get(2).unwrap().as_str();

            match op {
                "+" => println!("=> {}", elem1 + elem2),
                "-" => println!("=> {}", elem1 - elem2),
                "*" => println!("=> {}", elem1 * elem2),
                "/" => println!("=> {}", elem1 / elem2),
                _ => println!("Operación no reconocida"),
            }

            continue
        }

        println!("Expresión no reconocida. Escriba 'help' para obtener ayuda.");
    }
}
