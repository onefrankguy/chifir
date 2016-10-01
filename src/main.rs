mod machine;

use std::io::{self, Write};

fn main() {
    println!("Welcome to Chifir!");
    println!("");
    println!("Type 'help' to get started.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        let (tx, rx) = machine::spawn();

        io::stdin().read_line(&mut command).expect("Failed to read command");

        match command.trim() {
            "help" => println!("help, quit"),
            "step" => {
                tx.send(machine::Message::Step).unwrap();
                tx.send(machine::Message::Inspect).unwrap();
                println!("{}", rx.recv().unwrap());
            },
            "quit" => break,
            _ => continue
        }
    }
}
