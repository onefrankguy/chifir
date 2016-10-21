extern crate chifir;
extern crate termion;

use termion::raw::IntoRawMode;
use termion::async_stdin;

use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut stdout = Box::new(stdout.into_raw_mode().unwrap());

    let mut compiler = chifir::compiler::Compiler::new();
    write!(compiler, "{}", "
    ; Configure a 16x16 pixel display
    cfv display 10 10

    check-key:
      drw
      key x

      ; Exit if Ctrl+C is pressed
      sub y x ctrl-c
      beq /3 y exit

      ; Print 'A' if 'a' is pressed
      sub y x letter-a
      beq /3 y render-a

      ; Clear the display if anything else was pressed
      lpc /2 clear-display

    exit:
      brk

    ; Registers
    x:
      nop
    y:
      nop
    z:
      nop
    zz:
      nop
    k:
      nop
    kk:
      nop

    ; Constants
    ctrl-c:
      3
    letter-a:
      61
    one:
      1

    clear-display:
      lea k /3 100
      lea kk /3 display

    clear-display-loop:
      add x k kk
      sra /3 x 0
      beq /3 k check-key
      sub k k one
      lpc /2 clear-display-loop

    render-a:
      lea k /3 100
      lea kk /3 display
      lea zz /3 font-a

    render-a-loop:
      add x k kk
      add y k zz
      lra z y
      sra z x
      beq /3 k check-key
      sub k k one
      lpc /2 render-a-loop

    font-a:
      0 0 0 0
      0 1 1 1
      1 1 1 0
      0 0 0 0
      0 0 0 0
      0 1 1 1
      1 1 1 0
      0 0 0 0
      0 0 0 0
      0 1 1 1
      1 1 1 0
      0 0 0 0
      0 0 1 1
      1 0 0 0
      0 0 0 1
      1 1 0 0
      0 0 1 1
      1 0 0 0
      0 0 0 1
      1 1 0 0
      0 0 1 1
      1 0 0 0
      0 0 0 1
      1 1 0 0
      0 0 1 1
      1 1 1 1
      1 1 1 1
      1 1 0 0
      0 0 1 1
      1 1 1 1
      1 1 1 1
      1 1 0 0
      0 0 1 1
      1 1 1 1
      1 1 1 1
      1 1 0 0
      0 0 1 1
      1 0 0 0
      0 0 0 1
      1 1 0 0
      0 0 1 1
      1 0 0 0
      0 0 0 1
      1 1 0 0
      0 0 1 1
      1 0 0 0
      0 0 0 1
      1 1 0 0
      0 0 1 1
      1 0 0 0
      0 0 0 1
      1 1 0 0
      0 0 1 1
      1 0 0 0
      0 0 0 1
      1 1 0 0
      0 0 1 1
      1 0 0 0
      0 0 0 1
      1 1 0 0

    display:
      brk
    ").unwrap();
    compiler.compile();


    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1))
        .unwrap();
    stdout.flush().unwrap();

    let stdin = Box::new(async_stdin());

    let mut vm = chifir::computer::Computer::new().input(stdin).output(stdout);
    vm.load(compiler.bytecodes);

    while vm.next() != 0 {
        vm.step();
    }
}
