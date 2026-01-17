use std::io::*;
use parser::parse;
use executer::execute;

fn main() {
    loop {
        print!("$ ");
        let _ = stdout().flush();

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                match parse(&input) {
                    Ok(commands) => {
                        // Exécuter les commandes
                        execute(&commands);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
            Err(_) => eprintln!("Error reading input"),
        }
    }
}