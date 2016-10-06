extern crate chifir;
extern crate termion;

use termion::raw::IntoRawMode;
use termion::async_stdin;

use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = async_stdin();

    write!(stdout, "{}", termion::clear::All).unwrap();

    let mut vm = chifir::machine::Machine {
        memory: vec![
            14,0,0,1,   // 00 - Clear the screen
            15,6,0,3,   // 04 - Read key press and store it in $6
            8,3,7,6,    // 08 - $3 <- $7 - $6
            2,15,3,20,  // 12 - If $3 = 0, then PC <- 20
            1,19,0,4,   // 16 - Else PC <- 4
            0,0,0,0,    // 20 - Break
        ],
        counter: 0,
        output: stdout,
        input: stdin,
    };

    vm.step();
    while vm.next() != 0 {
        vm.step();
    }
}
