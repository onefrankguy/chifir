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

    let mut compiler = chifir::compiler::Compiler::new();
    compiler.parse("
    f 2 0 3  ; 00 - Read key press and store it in $2
    8 2 2 3  ; 04 - $2 <- $2 - $3
    2 b 2 f  ; 08 - If $2 = 0, then PC <- 15
    1 f 0 0  ; 12 - Else PC <- 0
    ");

    let mut vm = chifir::computer::Computer::with_io(stdin, stdout);
    vm.load(compiler.bytecodes);

    while vm.next() != 0 {
        vm.step();
    }
}
