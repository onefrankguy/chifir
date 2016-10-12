extern crate chifir;
extern crate termion;

use termion::raw::IntoRawMode;
use termion::async_stdin;

use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = async_stdin();

    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1))
        .unwrap();
    stdout.flush().unwrap();

    let mut vm = chifir::machine::Machine {
        memory: vec![
            0xf, 0x2, 0x0, 0x3, // 00 - Read key press and store it in $2
            0x8, 0x2, 0x2, 0x3, // 04 - $2 <- $2 - $3
            0x2, 0xb, 0x2, 0xf, // 08 - If $2 = 0, then PC <- 15
            0x1, 0xf, 0x0, 0x0, // 12 - Else PC <- 0
        ],
        counter: 0,
        output: stdout,
        input: stdin,
    };

    while vm.next() != 0 {
        vm.step();
    }
}
