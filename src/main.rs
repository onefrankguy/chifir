extern crate chifir;

use std::io::{self, Write};

fn main() {
    println!("Welcome to Chifir!");
    println!("");
    println!("Type 'help' to get started.");

    let (tx, rx) = chifir::machine::spawn();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut command = String::new();

        io::stdin().read_line(&mut command).expect("Failed to read command");

        match command.trim() {
            "help" => {
                println!("help, pause, resume, step, inspect, quit");
            }

            "pause" => {
                tx.send(chifir::machine::Message::Pause).unwrap();
            }

            "resume" => {
                tx.send(chifir::machine::Message::Resume).unwrap();
            }

            "step" => {
                tx.send(chifir::machine::Message::Step).unwrap();
                tx.send(chifir::machine::Message::Inspect).unwrap();
                println!("{}", rx.recv().unwrap());
            }

            "inspect" => {
                tx.send(chifir::machine::Message::Inspect).unwrap();
                println!("{}", rx.recv().unwrap());
            }

            "quit" => {
                break;
            }

            _ => {
                continue;
            }
        }
    }
}
