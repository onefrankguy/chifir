//! The `chifir::compiler` crate provides functions for transforming assembly
//! into Chifir bytecodes.
//!
//! # Examples
//!
//! This is the smallest Chifir program that does something useful. It exits
//! when //! <key>Ctrl-C</key> is pressed.
//!
//! ```
//! let mut compiler = chifir::compiler::Compiler::new();
//!
//! compiler.parse("
//! f 2 0 3
//! 8 2 2 3
//! 2 b 2 f
//! 1 e 0 0
//! ");
//!
//! assert_eq!(compiler.bytecodes, vec![
//! 0xf, 0x2, 0x0, 0x3,
//! 0x8, 0x2, 0x2, 0x3,
//! 0x2, 0xb, 0x2, 0xf,
//! 0x1, 0xe, 0x0, 0x0
//! ]);
//! ```
//!
//! Because raw machine code is hard to read, Chifir programs can include comments.
//! Comments start with a semicolon and go to the end of the line. Here's the above
//! program with comments.
//!
//! ```
//! let mut compiler = chifir::compiler::Compiler::new();
//!
//! compiler.parse("
//! f 2 0 3  ; Read key press and store it in M[2]
//! 8 2 2 3  ; Subtract M[3] from M[2] and store the result in M[2]
//! 2 b 2 f  ; If M[2] equals 0, then set PC to M[b]
//! 1 e 0 0  ; Else, set PC to M[e]
//! ");
//!
//! assert_eq!(compiler.bytecodes, vec![
//! 0xf, 0x2, 0x0, 0x3,
//! 0x8, 0x2, 0x2, 0x3,
//! 0x2, 0xb, 0x2, 0xf,
//! 0x1, 0xe, 0x0, 0x0
//! ]);
//! ```
//!
//! The term **PC** in the comments refers to the program counter. Chifir starts
//! with the program counter at 0. The term **M[X]** in the comments refers to the
//! **X**<sup>th</sup> location in memory. Memory is allocated when it's accessed,
//! and Chifir programs can use up to 16 GiB of memory.
//!
//! Every opcode and operand in a Chifir program is 32 bits. This Chifir program is
//! written in machine code with hexadecimal values for the opcodes and operands.
//! Because hex values for opcodes are hard to memorize, Chifir programs can use
//! three letter abbreviations for the opcodes instead.
//!
//! ```
//! let mut compiler = chifir::compiler::Compiler::new();
//!
//! compiler.parse("
//! key 2 0 3  ; Read key press and store it in M[2]
//! sub 2 2 3  ; Subtract M[3] from M[2] and store the result in M[2]
//! beq b 2 f  ; If M[2] equals 0, then set PC to M[b]
//! lpc e 0 0  ; Else, set PC to M[e]
//! ");
//!
//! assert_eq!(compiler.bytecodes, vec![
//! 0xf, 0x2, 0x0, 0x3,
//! 0x8, 0x2, 0x2, 0x3,
//! 0x2, 0xb, 0x2, 0xf,
//! 0x1, 0xe, 0x0, 0x0
//! ]);
//! ```

use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;

pub struct Compiler {
    pub lines: Vec<String>,
    pub instructions: Vec<String>,
    pub labels: HashMap<String, u32>,
    pub bytecodes: Vec<u32>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            lines: Vec::new(),
            instructions: Vec::new(),
            labels: HashMap::new(),
            bytecodes: Vec::new(),
        }
    }

    pub fn parse(&mut self, assembly: &str) {
        self.split_lines(assembly);
        self.strip_comments();
        self.compile_labels();
        self.compile_bytecodes();
    }

    /// This function transforms an opcode into a bytecode. The following tabl
    /// lists all the valid opcodes.
    ///
    /// |Opcode|Abbreviation|Semantics                                                |
    /// |:----:|:-----------|:--------------------------------------------------------|
    /// |0     |`brk`       |Halt execution                                           |
    /// |1     |`lpc`       |PC &larr; M[A]                                           |
    /// |2     |`beq`       |If M[B] &equals; 0, then PC &larr; M[A]                  |
    /// |3     |`spc`       |M[A] &larr; PC                                           |
    /// |4     |`lea`       |M[A] &larr; M[B]                                         |
    /// |5     |`lra`       |M[A] &larr; M[M[B]]                                      |
    /// |6     |`sra`       |M[M[B]] &larr; M[A]                                      |
    /// |7     |`add`       |M[A] &larr; M[B] &plus; M[C]                             |
    /// |8     |`sub`       |M[A] &larr; M[B] &minus; M[C]                            |
    /// |9     |`mul`       |M[A] &larr; M[B] &times; M[C]                            |
    /// |10    |`div`       |M[A] &larr; M[B] &divide; M[C]                           |
    /// |11    |`mod`       |M[A] &larr; M[B] modulo M[C]                             |
    /// |12    |`cmp`       |If M[B] &lt; M[C], then M[A] &larr; 1, else M[A] &larr; 0|
    /// |13    |`nad`       |M[A] &larr; NOT(M[B} AND M[C])                           |
    /// |14    |`drw`       |Refresh the screen                                       |
    /// |15    |`key`       |Get the last key pressed and store it in M[A]            |
    /// |16    |`nop`       |Skip this instruction                                    |
    ///
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

#[cfg(test)]
mod tests {
    use super::Compiler;
    use std::collections::HashMap;

    #[test]
    fn it_splits_lines_by_line_feed() {
        let mut compiler = Compiler::new();
        compiler.parse("0 0 0 0\n0 0 0 0");

        assert_eq!(compiler.lines.len(), 2);
    }

    #[test]
    fn it_splits_lines_by_vertical_tab() {
        let mut compiler = Compiler::new();
        compiler.parse("0 0 0 0\x0B0 0 0 0");

        assert_eq!(compiler.lines.len(), 2);
    }

    #[test]
    fn it_splits_lines_by_form_feed() {
        let mut compiler = Compiler::new();
        compiler.parse("0 0 0 0\x0C0 0 0 0");

        assert_eq!(compiler.lines.len(), 2);
    }

    #[test]
    fn it_splits_lines_by_carriage_return() {
        let mut compiler = Compiler::new();
        compiler.parse("0 0 0 0\r0 0 0 0");

        assert_eq!(compiler.lines.len(), 2);
    }

    #[test]
    fn it_splits_lines_by_carriage_return_line_feed() {
        let mut compiler = Compiler::new();
        compiler.parse("0 0 0 0\r\n0 0 0 0");

        assert_eq!(compiler.lines.len(), 2);
    }

    #[test]
    fn it_splits_lines_by_next_line() {
        let mut compiler = Compiler::new();
        compiler.parse("0 0 0 0\u{0085}0 0 0 0");

        assert_eq!(compiler.lines.len(), 2);
    }

    #[test]
    fn it_splits_lines_by_line_separator() {
        let mut compiler = Compiler::new();
        compiler.parse("0 0 0 0\u{2028}0 0 0 0");

        assert_eq!(compiler.lines.len(), 2);
    }

    #[test]
    fn it_splits_lines_by_paragraph_separator() {
        let mut compiler = Compiler::new();
        compiler.parse("0 0 0 0\u{2029}0 0 0 0");

        assert_eq!(compiler.lines.len(), 2);
    }

    #[test]
    fn it_ignors_trailing_separators() {
        let mut compiler = Compiler::new();
        compiler.parse("0 0 0 0\r\n0 0 0 0\r\n");

        assert_eq!(compiler.lines.len(), 2);
    }

    #[test]
    fn it_strips_single_line_comments() {
        let mut compiler = Compiler::new();
        compiler.parse("; single line comment\n0 0 0 0\n");

        assert_eq!(compiler.lines.len(), 2);
        assert_eq!(compiler.instructions.len(), 1);
    }

    #[test]
    fn it_strips_single_line_comments_starting_with_spaces() {
        let mut compiler = Compiler::new();
        compiler.parse(" ; single line comment\n0 0 0 0\n");

        assert_eq!(compiler.lines.len(), 2);
        assert_eq!(compiler.instructions.len(), 1);
    }

    #[test]
    fn it_strips_single_line_comments_starting_with_tabs() {
        let mut compiler = Compiler::new();
        compiler.parse("\t; single line comment\n0 0 0 0\n");

        assert_eq!(compiler.lines.len(), 2);
        assert_eq!(compiler.instructions.len(), 1);
    }

    #[test]
    fn it_strips_inline_comments() {
        let mut compiler = Compiler::new();
        compiler.parse("0 0 0 0; inline comment\n");

        assert_eq!(compiler.lines.len(), 1);
        assert_eq!(compiler.instructions, vec!["0 0 0 0"]);
    }

    #[test]
    fn it_strips_inline_comments_starting_with_spaces() {
        let mut compiler = Compiler::new();
        compiler.parse("0 0 0 0 ; inline comment\n");

        assert_eq!(compiler.lines.len(), 1);
        assert_eq!(compiler.instructions, vec!["0 0 0 0"]);
    }

    #[test]
    fn it_strips_inline_comments_starting_with_tabs() {
        let mut compiler = Compiler::new();
        compiler.parse("0 0 0 0\t; inline comment\n");

        assert_eq!(compiler.lines.len(), 1);
        assert_eq!(compiler.instructions, vec!["0 0 0 0"]);
    }

    #[test]
    fn it_trims_spaces_from_instructions() {
        let mut compiler = Compiler::new();
        compiler.parse(" 0 0 0 0 ");

        assert_eq!(compiler.lines.len(), 1);
        assert_eq!(compiler.instructions, vec!["0 0 0 0"]);
    }

    #[test]
    fn it_trims_tabs_from_instructions() {
        let mut compiler = Compiler::new();
        compiler.parse("\t0 0 0 0\t");

        assert_eq!(compiler.lines.len(), 1);
        assert_eq!(compiler.instructions, vec!["0 0 0 0"]);
    }

    #[test]
    fn it_compiles_addresses_for_labels() {
        let mut compiler = Compiler::new();
        compiler.parse("first:\nsecond:\n0 0 0 0\nthird:");

        let mut labels = HashMap::new();
        labels.insert("first".to_string(), 0);
        labels.insert("second".to_string(), 0);
        labels.insert("third".to_string(), 4);

        assert_eq!(compiler.labels, labels);
    }

    #[test]
    fn it_lets_the_last_label_win() {
        let mut compiler = Compiler::new();
        compiler.parse("label:\n0 0 0 0\nlabel:");

        let mut labels = HashMap::new();
        labels.insert("label".to_string(), 4);

        assert_eq!(compiler.labels, labels);
    }

    #[test]
    fn it_ignores_trailing_label_characters() {
        let mut compiler = Compiler::new();
        compiler.parse("label:with bits\n0 0 0 0\nlabel:with bytes");

        let mut labels = HashMap::new();
        labels.insert("label".to_string(), 4);

        assert_eq!(compiler.labels, labels);
    }

    #[test]
    fn it_parses_hex_for_opcode_0() {
        let mut compiler = Compiler::new();
        compiler.parse("0");

        assert_eq!(compiler.bytecodes, vec![0x0, 0x0, 0x0, 0x0]);
    }

    #[test]
    fn it_parses_brk_as_opcode_0() {
        let mut compiler = Compiler::new();
        compiler.parse("brk");

        assert_eq!(compiler.bytecodes, vec![0x0, 0x0, 0x0, 0x0]);
    }

    #[test]
    fn it_parses_operand_a_as_hex() {
        let mut compiler = Compiler::new();
        compiler.parse("brk f");

        assert_eq!(compiler.bytecodes, vec![0x0, 0xf, 0x0, 0x0]);
    }

    #[test]
    fn it_parses_operand_a_as_a_label() {
        let mut compiler = Compiler::new();
        compiler.parse("brk end\nend:");

        assert_eq!(compiler.bytecodes, vec![0x0, 0x4, 0x0, 0x0]);
    }

    #[test]
    fn it_parses_operand_b_as_hex() {
        let mut compiler = Compiler::new();
        compiler.parse("brk f f");

        assert_eq!(compiler.bytecodes, vec![0x0, 0xf, 0xf, 0x0]);
    }

    #[test]
    fn it_parses_operand_b_as_a_label() {
        let mut compiler = Compiler::new();
        compiler.parse("brk f end\nend:");

        assert_eq!(compiler.bytecodes, vec![0x0, 0xf, 0x4, 0x0]);
    }

    #[test]
    fn it_parses_operand_c_as_hex() {
        let mut compiler = Compiler::new();
        compiler.parse("brk f f f");

        assert_eq!(compiler.bytecodes, vec![0x0, 0xf, 0xf, 0xf]);
    }

    #[test]
    fn it_parses_operand_c_as_a_label() {
        let mut compiler = Compiler::new();
        compiler.parse("brk f f end\nend:");

        assert_eq!(compiler.bytecodes, vec![0x0, 0xf, 0xf, 0x4]);
    }

    #[test]
    fn it_parses_operands_as_relative_addresses() {
        let mut compiler = Compiler::new();
        compiler.parse("nop\nadd /0 /5 /6");

        assert_eq!(compiler.bytecodes,
                   vec![0x10, 0x0, 0x0, 0x0, 0x7, 0x4, 0x9, 0xa]);
    }

    #[test]
    fn it_compiles_the_simple_ctrl_c_example() {
        let mut compiler = Compiler::new();

        compiler.parse("
        f 2 0 3  ; Read key press and store it in M[2]
        8 2 2 3  ; Subtract M[3] from M[2] and store the result in M[2]
        2 b 2 f  ; If M[2] equals 0, then set PC to M[b]
        1 e 0 0  ; Else, set PC to M[e]
        ");

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

        compiler.parse("
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
        ");

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
