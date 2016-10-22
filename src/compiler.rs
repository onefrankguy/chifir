//! A compiler for transforming assembly into bytecodes.
//!
//! This is the smallest Chifir program that does something useful. It exits
//! when <kbd>Ctrl</kbd> + <kbd>C</kbd> is pressed.
//!
//! ```
//! use std::io::Write;
//! use chifir::compiler::Compiler;
//! use chifir::computer::Computer;
//!
//! let mut compiler = Compiler::new();
//!
//! write!(compiler, "{}","
//! f 2 0 3
//! 8 2 2 3
//! 2 b 2 f
//! 1 e 0 0
//! ").unwrap();
//!
//! let bytecodes = compiler.compile().unwrap();
//!
//! assert_eq!([
//! 0xf, 0x2, 0x0, 0x3,
//! 0x8, 0x2, 0x2, 0x3,
//! 0x2, 0xb, 0x2, 0xf,
//! 0x1, 0xe, 0x0, 0x0
//! ], bytecodes);
//! ```
//!
//! # Comments
//!
//! Because raw machine code is hard to read, programs can include comments.
//! Comments start with a semicolon and go to the end of the line.
//!
//! ```
//! use std::io::Write;
//! use chifir::compiler::Compiler;
//!
//! let mut compiler = Compiler::new();
//!
//! write!(compiler, "{}","
//! f 2 0 3  ; Read key press and store it in M[2]
//! 8 2 2 3  ; Subtract M[3] from M[2] and store the result in M[2]
//! 2 b 2 f  ; If M[2] equals 0, then set PC to M[b]
//! 1 e 0 0  ; Else, set PC to M[e]
//! ").unwrap();
//!
//! let bytecodes = compiler.compile().unwrap();
//!
//! assert_eq!([
//! 0xf, 0x2, 0x0, 0x3,
//! 0x8, 0x2, 0x2, 0x3,
//! 0x2, 0xb, 0x2, 0xf,
//! 0x1, 0xe, 0x0, 0x0
//! ], bytecodes);
//! ```
//!
//! The term **PC** in the comments refers to the program counter. Chifir starts
//! with the program counter at 0. The term **M[X]** in the comments refers to
//! the **X**<sup>th</sup> location in memory. Memory is allocated when it's
//! accessed, and programs can use up to 16 GiB of memory.
//!
//! # Instructions
//!
//! Chifir instructions consist of an opcode followed by three operands. Both
//! opcodes and operands are 32 bits and are written in hexadecimal. Because hex
//! values for opcodes are hard to memorize, programs can use three letter
//! abbreviations for opcodes instead.
//!
//! ```
//! use std::io::Write;
//! use chifir::compiler::Compiler;
//!
//! let mut compiler = Compiler::new();
//!
//! write!(compiler, "{}","
//! key 2 0 3  ; Read key press and store it in M[2]
//! sub 2 2 3  ; Subtract M[3] from M[2] and store the result in M[2]
//! beq b 2 f  ; If M[2] equals 0, then set PC to M[b]
//! lpc e 0 0  ; Else, set PC to M[e]
//! ").unwrap();
//!
//! let bytecodes = compiler.compile().unwrap();
//!
//! assert_eq!([
//! 0xf, 0x2, 0x0, 0x3,
//! 0x8, 0x2, 0x2, 0x3,
//! 0x2, 0xb, 0x2, 0xf,
//! 0x1, 0xe, 0x0, 0x0
//! ], bytecodes);
//! ```
//!
//! [Table 1](#table-1) has a full list of opcodes and their abbreviations.
//!
//!
//! # Labels
//!
//! Labels can be used as references to locations in a program. Labels have two
//! parts, definitions and references. A label definition is on a line by
//! itself and ends with a semicolon. A label reference is used as an operand in
//! an instruction.
//!
//! When the compiler finds a label reference, it replaces the reference with
//! the location in memory of the first instruction that came after the label
//! was defined.
//!
//! ```
//! use std::io::Write;
//! use chifir::compiler::Compiler;
//!
//! let mut compiler = Compiler::new();
//!
//! write!(compiler, "{}","
//! ; Halt when Ctrl+C is pressed.
//!
//! check-ctrl-c:
//!   key 2 0 3
//!   sub 2 2 3
//!   beq b 2 exit
//!   lpc e check-ctrl-c 0
//!
//! exit:
//!   brk 0 0 0
//! ").unwrap();
//!
//! let bytecodes = compiler.compile().unwrap();
//!
//! assert_eq!([
//! 0xf, 0x2, 0x0, 0x3,
//! 0x8, 0x2, 0x2, 0x3,
//! 0x2, 0xb, 0x2, 0x10,
//! 0x1, 0xe, 0x0, 0x0,
//! 0x0, 0x0, 0x0, 0x0
//! ], bytecodes);
//! ```
//!
//! Labels can be referenced before they're defined. This makes them useful for
//! declaring storage locations and constants.
//!
//! ```
//! use std::io::Write;
//! use chifir::compiler::Compiler;
//!
//! let mut compiler = Compiler::new();
//!
//! write!(compiler, "{}","
//! ; Halt when Ctrl+C is pressed.
//!
//! check-ctrl-c:
//!   key x 0 0
//!   sub x x ctrl-c
//!   beq b x exit
//!   lpc e check-ctrl-c 0
//!
//! exit:
//!   brk 0 0 0
//!
//! x:
//!   nop 0 0 0
//!
//! ctrl-c:
//!   lea 18 1b 3
//! ").unwrap();
//!
//! let bytecodes = compiler.compile().unwrap();
//!
//! assert_eq!([
//! 0xf, 0x14, 0x0, 0x0,
//! 0x8, 0x14, 0x14, 0x18,
//! 0x2, 0xb, 0x14, 0x10,
//! 0x1, 0xe, 0x0, 0x0,
//! 0x0, 0x0, 0x0, 0x0,
//! 0x10, 0x0, 0x0, 0x0,
//! 0x4, 0x18, 0x1b, 0x3
//! ], bytecodes);
//! ```
//!
//!
//! # Table 1
//!
//! A complete list of all Chifir opcodes.
//!
//! |Opcode|Abbreviation|Semantics                                                |
//! |:----:|:-----------|:--------------------------------------------------------|
//! |0     |`brk`       |Halt execution                                           |
//! |1     |`lpc`       |PC &larr; M[A]                                           |
//! |2     |`beq`       |If M[B] &equals; 0, then PC &larr; M[A]                  |
//! |3     |`spc`       |M[A] &larr; PC                                           |
//! |4     |`lea`       |M[A] &larr; M[B]                                         |
//! |5     |`lra`       |M[A] &larr; M[M[B]]                                      |
//! |6     |`sra`       |M[M[B]] &larr; M[A]                                      |
//! |7     |`add`       |M[A] &larr; M[B] &plus; M[C]                             |
//! |8     |`sub`       |M[A] &larr; M[B] &minus; M[C]                            |
//! |9     |`mul`       |M[A] &larr; M[B] &times; M[C]                            |
//! |10    |`div`       |M[A] &larr; M[B] &divide; M[C]                           |
//! |11    |`mod`       |M[A] &larr; M[B] modulo M[C]                             |
//! |12    |`cmp`       |If M[B] &lt; M[C], then M[A] &larr; 1, else M[A] &larr; 0|
//! |13    |`nad`       |M[A] &larr; NOT(M[B} AND M[C])                           |
//! |14    |`drw`       |Refresh the screen                                       |
//! |15    |`key`       |Get the last key pressed and store it in M[A]            |
//! |16    |`nop`       |Skip this instruction                                    |
//! |17    |`cfv`       |Configure display at M[A] with width B and height C      |

use std::vec::Vec;
use std::string::{self, String};
use std::collections::HashMap;
use std::io::{self, Write};

pub struct Compiler {
    assembly: Vec<u8>,
    lines: Vec<String>,
    instructions: Vec<String>,
    labels: HashMap<String, u32>,
    bytecodes: Vec<u32>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            assembly: Vec::new(),
            lines: Vec::new(),
            instructions: Vec::new(),
            labels: HashMap::new(),
            bytecodes: Vec::new(),
        }
    }

    pub fn compile(&mut self) -> Result<&[u32], CompilerError> {
        let assembly = try!(String::from_utf8(self.assembly.to_vec()).map_err(CompilerError::FromUtf8Error));
        self.split_lines(assembly.as_str());
        self.strip_comments();
        self.compile_labels();
        self.compile_bytecodes();
        Ok(self.bytecodes.as_slice())
    }

    // Transform an opcode into a bytecode. Undefined opcodes, or thoses that
    // fail to parse, are treated as zero. This maintains the concept of all
    // uninitialized memory being zeroed out.
    fn parse_opcode(&self, opcode: Option<&str>) -> u32 {
        match opcode {
            Some("brk") => 0,
            Some("lpc") => 1,
            Some("beq") => 2,
            Some("spc") => 3,
            Some("lea") => 4,
            Some("lra") => 5,
            Some("sra") => 6,
            Some("add") => 7,
            Some("sub") => 8,
            Some("mul") => 9,
            Some("div") => 10,
            Some("mod") => 11,
            Some("cmp") => 12,
            Some("nad") => 13,
            Some("drw") => 14,
            Some("key") => 15,
            Some("nop") => 16,
            Some("cfv") => 17,
            Some(opcode) => u32::from_str_radix(opcode, 16).unwrap_or(0),
            None => 0,
        }
    }

    // Transform an operand into a bytecode. Undefined operands, or thoses that
    // fail to parse, are treated as zero. This maintains the concept of all
    // uninitialized memory being zeroed out.
    fn parse_operand(&self, operand: Option<&str>, opcode_address: u32) -> u32 {
        match operand {
            Some(operand) => {
                match self.labels.get(operand) {
                    // Operand is a label with an absolute address
                    Some(address) => *address,

                    None => {
                        // Operand is a relative address
                        if operand.starts_with('/') {
                            let address = operand.trim_left_matches('/');
                            let address = u32::from_str_radix(address, 16).unwrap_or(0);
                            address + opcode_address
                        }
                        // Operand is a numeric value
                        else {
                            u32::from_str_radix(operand, 16).unwrap_or(0)
                        }
                    }
                }
            }
            None => 0,
        }
    }

    fn compile_bytecodes(&mut self) {
        let mut instructions = self.instructions.iter();

        while let Some(instruction) = instructions.next() {
            match instruction.find(':') {
                Some(_) => {
                    // Ignore labels
                }
                None => {
                    let opcode_address = self.bytecodes.len() as u32;
                    let mut bytecodes = instruction.split_whitespace();

                    // Opcode
                    let opcode = bytecodes.next();
                    let opcode = self.parse_opcode(opcode);
                    self.bytecodes.push(opcode);

                    // Operand A
                    let operand_a = bytecodes.next();
                    let operand_a = self.parse_operand(operand_a, opcode_address);
                    self.bytecodes.push(operand_a);

                    // Operand B
                    let operand_b = bytecodes.next();
                    let operand_b = self.parse_operand(operand_b, opcode_address);
                    self.bytecodes.push(operand_b);

                    // Operand C
                    let operand_c = bytecodes.next();
                    let operand_c = self.parse_operand(operand_c, opcode_address);
                    self.bytecodes.push(operand_c);
                }
            }
        }
    }

    fn compile_labels(&mut self) {
        let mut instructions = self.instructions.iter();
        let mut address = 0;

        while let Some(instruction) = instructions.next() {
            match instruction.find(':') {
                Some(index) => {
                    self.labels.insert(instruction.split_at(index).0.to_string(), address);
                }
                None => {
                    address += 4;
                }
            }
        }
    }

    fn strip_comments(&mut self) {
        let mut lines = self.lines.iter();
        while let Some(line) = lines.next() {
            let trimmed_line = line.trim();
            if !trimmed_line.is_empty() && !trimmed_line.starts_with(";") {
                let instruction = match trimmed_line.find(';') {
                    Some(index) => trimmed_line.split_at(index).0,
                    None => trimmed_line,
                };
                self.instructions.push(instruction.trim().to_string());
            }
        }
    }

    fn split_lines(&mut self, assembly: &str) {
        let mut line = String::new();
        let mut chars = assembly.chars();

        while let Some(c) = chars.next() {
            match c {
                // Line Feed | Vertical Tab | Form Feed | Next Line | Line/Paragraph Separator
                '\u{000A}' | '\u{000B}' | '\u{000C}' | '\u{0085}' | '\u{2028}' | '\u{2029}' => {
                    self.lines.push(line);
                    line = String::new();
                }

                // Carriage Return
                '\u{000D}' => {
                    self.lines.push(line);
                    line = String::new();

                    // Carriage Return + Line Feed
                    match chars.next() {
                        Some(n) => {
                            if n != '\u{000A}' {
                                line.push(n);
                            }
                        }
                        None => {}
                    }
                }

                _ => {
                    line.push(c);
                }
            }
        }

        if !line.is_empty() {
            self.lines.push(line);
        }
    }
}

impl Write for Compiler {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.assembly.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.assembly.flush()
    }
}

#[derive(Debug)]
pub enum CompilerError {
    FromUtf8Error(string::FromUtf8Error),
}

#[cfg(test)]
mod tests {
    use super::Compiler;
    use std::collections::HashMap;
    use std::io::Write;

    #[test]
    fn it_splits_lines_by_line_feed() {
        let mut compiler = Compiler::new();
        compiler.write(b"0 0 0 0\n0 0 0 0").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.lines.len(), 2);
    }

    #[test]
    fn it_splits_lines_by_vertical_tab() {
        let mut compiler = Compiler::new();
        compiler.write(b"0 0 0 0\x0B0 0 0 0").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.lines.len(), 2);
    }

    #[test]
    fn it_splits_lines_by_form_feed() {
        let mut compiler = Compiler::new();
        compiler.write(b"0 0 0 0\x0C0 0 0 0").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.lines.len(), 2);
    }

    #[test]
    fn it_splits_lines_by_carriage_return() {
        let mut compiler = Compiler::new();
        compiler.write(b"0 0 0 0\r0 0 0 0").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.lines.len(), 2);
    }

    #[test]
    fn it_splits_lines_by_carriage_return_line_feed() {
        let mut compiler = Compiler::new();
        compiler.write(b"0 0 0 0\r\n0 0 0 0").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.lines.len(), 2);
    }

    #[test]
    fn it_splits_lines_by_next_line() {
        let mut compiler = Compiler::new();
        write!(compiler, "{}", "0 0 0 0\u{0085}0 0 0 0").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.lines.len(), 2);
    }

    #[test]
    fn it_splits_lines_by_line_separator() {
        let mut compiler = Compiler::new();
        write!(compiler, "{}", "0 0 0 0\u{2028}0 0 0 0").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.lines.len(), 2);
    }

    #[test]
    fn it_splits_lines_by_paragraph_separator() {
        let mut compiler = Compiler::new();
        write!(compiler, "{}", "0 0 0 0\u{2029}0 0 0 0").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.lines.len(), 2);
    }

    #[test]
    fn it_ignors_trailing_separators() {
        let mut compiler = Compiler::new();
        compiler.write(b"0 0 0 0\r\n0 0 0 0\r\n").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.lines.len(), 2);
    }

    #[test]
    fn it_strips_single_line_comments() {
        let mut compiler = Compiler::new();
        compiler.write(b"; single line comment\n0 0 0 0\n").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.lines.len(), 2);
        assert_eq!(compiler.instructions.len(), 1);
    }

    #[test]
    fn it_strips_single_line_comments_starting_with_spaces() {
        let mut compiler = Compiler::new();
        compiler.write(b" ; single line comment\n0 0 0 0\n").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.lines.len(), 2);
        assert_eq!(compiler.instructions.len(), 1);
    }

    #[test]
    fn it_strips_single_line_comments_starting_with_tabs() {
        let mut compiler = Compiler::new();
        compiler.write(b"\t; single line comment\n0 0 0 0\n").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.lines.len(), 2);
        assert_eq!(compiler.instructions.len(), 1);
    }

    #[test]
    fn it_strips_inline_comments() {
        let mut compiler = Compiler::new();
        compiler.write(b"0 0 0 0; inline comment\n").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.lines.len(), 1);
        assert_eq!(compiler.instructions, vec!["0 0 0 0"]);
    }

    #[test]
    fn it_strips_inline_comments_starting_with_spaces() {
        let mut compiler = Compiler::new();
        compiler.write(b"0 0 0 0 ; inline comment\n").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.lines.len(), 1);
        assert_eq!(compiler.instructions, vec!["0 0 0 0"]);
    }

    #[test]
    fn it_strips_inline_comments_starting_with_tabs() {
        let mut compiler = Compiler::new();
        compiler.write(b"0 0 0 0\t; inline comment\n").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.lines.len(), 1);
        assert_eq!(compiler.instructions, vec!["0 0 0 0"]);
    }

    #[test]
    fn it_trims_spaces_from_instructions() {
        let mut compiler = Compiler::new();
        compiler.write(b" 0 0 0 0 ").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.lines.len(), 1);
        assert_eq!(compiler.instructions, vec!["0 0 0 0"]);
    }

    #[test]
    fn it_trims_tabs_from_instructions() {
        let mut compiler = Compiler::new();
        compiler.write(b"\t0 0 0 0\t").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.lines.len(), 1);
        assert_eq!(compiler.instructions, vec!["0 0 0 0"]);
    }

    #[test]
    fn it_compiles_addresses_for_labels() {
        let mut compiler = Compiler::new();
        compiler.write(b"first:\nsecond:\n0 0 0 0\nthird:").unwrap();
        compiler.compile().unwrap();

        let mut labels = HashMap::new();
        labels.insert("first".to_string(), 0);
        labels.insert("second".to_string(), 0);
        labels.insert("third".to_string(), 4);

        assert_eq!(compiler.labels, labels);
    }

    #[test]
    fn it_lets_the_last_label_win() {
        let mut compiler = Compiler::new();
        compiler.write(b"label:\n0 0 0 0\nlabel:").unwrap();
        compiler.compile().unwrap();

        let mut labels = HashMap::new();
        labels.insert("label".to_string(), 4);

        assert_eq!(compiler.labels, labels);
    }

    #[test]
    fn it_ignores_trailing_label_characters() {
        let mut compiler = Compiler::new();
        compiler.write(b"label:with bits\n0 0 0 0\nlabel:with bytes").unwrap();
        compiler.compile().unwrap();

        let mut labels = HashMap::new();
        labels.insert("label".to_string(), 4);

        assert_eq!(compiler.labels, labels);
    }

    #[test]
    fn it_parses_hex_for_opcode_0() {
        let mut compiler = Compiler::new();
        compiler.write(b"0").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.bytecodes, vec![0x0, 0x0, 0x0, 0x0]);
    }

    #[test]
    fn it_parses_brk_as_opcode_0() {
        let mut compiler = Compiler::new();
        compiler.write(b"brk").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.bytecodes, vec![0x0, 0x0, 0x0, 0x0]);
    }

    #[test]
    fn it_parses_operand_a_as_hex() {
        let mut compiler = Compiler::new();
        compiler.write(b"brk f").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.bytecodes, vec![0x0, 0xf, 0x0, 0x0]);
    }

    #[test]
    fn it_parses_operand_a_as_a_label() {
        let mut compiler = Compiler::new();
        compiler.write(b"brk end\nend:").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.bytecodes, vec![0x0, 0x4, 0x0, 0x0]);
    }

    #[test]
    fn it_parses_operand_b_as_hex() {
        let mut compiler = Compiler::new();
        compiler.write(b"brk f f").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.bytecodes, vec![0x0, 0xf, 0xf, 0x0]);
    }

    #[test]
    fn it_parses_operand_b_as_a_label() {
        let mut compiler = Compiler::new();
        compiler.write(b"brk f end\nend:").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.bytecodes, vec![0x0, 0xf, 0x4, 0x0]);
    }

    #[test]
    fn it_parses_operand_c_as_hex() {
        let mut compiler = Compiler::new();
        compiler.write(b"brk f f f").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.bytecodes, vec![0x0, 0xf, 0xf, 0xf]);
    }

    #[test]
    fn it_parses_operand_c_as_a_label() {
        let mut compiler = Compiler::new();
        compiler.write(b"brk f f end\nend:").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.bytecodes, vec![0x0, 0xf, 0xf, 0x4]);
    }

    #[test]
    fn it_parses_operands_as_relative_addresses() {
        let mut compiler = Compiler::new();
        compiler.write(b"nop\nadd /0 /5 /6").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.bytecodes,
                   vec![0x10, 0x0, 0x0, 0x0, 0x7, 0x4, 0x9, 0xa]);
    }

    #[test]
    fn it_compiles_the_simple_ctrl_c_example() {
        let mut compiler = Compiler::new();

        compiler.write(b"
        f 2 0 3  ; Read key press and store it in M[2]
        8 2 2 3  ; Subtract M[3] from M[2] and store the result in M[2]
        2 b 2 f  ; If M[2] equals 0, then set PC to M[b]
        1 e 0 0  ; Else, set PC to M[e]
        ").unwrap();
        compiler.compile().unwrap();

        assert_eq!(compiler.bytecodes,
                   vec![
            0xf, 0x2, 0x0, 0x3,
            0x8, 0x2, 0x2, 0x3,
            0x2, 0xb, 0x2, 0xf,
            0x1, 0xe, 0x0, 0x0,
        ]);
    }

    #[test]
    fn it_compiles_the_complex_ctrl_c_example() {
        let mut compiler = Compiler::new();

        compiler.write(b"
        check-ctrl-c:
          key x                ; Read key press and store it in M[x]
          sub x x ctrl-c       ; Subtract M[ctrl-c] from M[x] and store the result in M[x]
          beq /3 x exit        ; If M[x] equals 0, then set PC to M[exit]
          lpc /2 check-ctrl-c  ; Else, set PC to M[check-ctrl-c]

        exit:
          brk

        x:
          nop

        ctrl-c:
          lea /0 /3 3
        ").unwrap();
        compiler.compile().unwrap();

        let mut labels = HashMap::new();
        labels.insert("check-ctrl-c".to_string(), 0);
        labels.insert("exit".to_string(), 16);
        labels.insert("x".to_string(), 20);
        labels.insert("ctrl-c".to_string(), 24);

        assert_eq!(compiler.labels, labels);

        assert_eq!(compiler.bytecodes,
                   vec![0xf, 0x14, 0x0, 0x0, 0x8, 0x14, 0x14, 0x18, 0x2, 0xb, 0x14, 0x10, 0x1,
                        0xe, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x10, 0x0, 0x0, 0x0, 0x4, 0x18, 0x1b,
                        0x3]);
    }
}
