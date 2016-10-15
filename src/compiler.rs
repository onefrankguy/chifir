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

    fn compile_bytecodes(&mut self) {
        let mut instructions = self.instructions.iter();

        while let Some(instruction) = instructions.next() {
            match instruction.find(':') {
                Some(_) => {
                    // Ignore labels
                }
                None => {
                    let mut bytecodes = instruction.split_whitespace();

                    // Opcode
                    match bytecodes.next() {
                        Some("brk") => {
                            self.bytecodes.push(0);
                        }
                        Some("lpc") => {
                            self.bytecodes.push(1);
                        }
                        Some("beq") => {
                            self.bytecodes.push(2);
                        }
                        Some("spc") => {
                            self.bytecodes.push(3);
                        }
                        Some("lea") => {
                            self.bytecodes.push(4);
                        }
                        Some("lra") => {
                            self.bytecodes.push(5);
                        }
                        Some("sra") => {
                            self.bytecodes.push(6);
                        }
                        Some("add") => {
                            self.bytecodes.push(7);
                        }
                        Some("sub") => {
                            self.bytecodes.push(8);
                        }
                        Some("mul") => {
                            self.bytecodes.push(9);
                        }
                        Some("div") => {
                            self.bytecodes.push(10);
                        }
                        Some("mod") => {
                            self.bytecodes.push(11);
                        }
                        Some("cmp") => {
                            self.bytecodes.push(12);
                        }
                        Some("nad") => {
                            self.bytecodes.push(13);
                        }
                        Some("drw") => {
                            self.bytecodes.push(14);
                        }
                        Some("key") => {
                            self.bytecodes.push(15);
                        }
                        Some("nop") => {
                            self.bytecodes.push(16);
                        }
                        Some(opcode) => {
                            match u32::from_str_radix(opcode, 16) {
                                Ok(bytecode) => {
                                    self.bytecodes.push(bytecode);
                                }
                                _ => {}
                            }
                        }
                        None => {}
                    }

                    // Operand A
                    match bytecodes.next() {
                        Some(operand) => {
                            match self.labels.get(operand) {
                                None => {
                                    match u32::from_str_radix(operand, 16) {
                                        Ok(bytecode) => {
                                            self.bytecodes.push(bytecode);
                                        }
                                        _ => {}
                                    }
                                }
                                Some(address) => {
                                    self.bytecodes.push(*address);
                                }
                            }
                        }
                        None => {}
                    }

                    // Operand B
                    match bytecodes.next() {
                        Some(operand) => {
                            match self.labels.get(operand) {
                                None => {
                                    match u32::from_str_radix(operand, 16) {
                                        Ok(bytecode) => {
                                            self.bytecodes.push(bytecode);
                                        }
                                        _ => {}
                                    }
                                }
                                Some(address) => {
                                    self.bytecodes.push(*address);
                                }
                            }
                        }
                        None => {}
                    }

                    // Operand C
                    match bytecodes.next() {
                        Some(operand) => {
                            match self.labels.get(operand) {
                                None => {
                                    match u32::from_str_radix(operand, 16) {
                                        Ok(bytecode) => {
                                            self.bytecodes.push(bytecode);
                                        }
                                        _ => {}
                                    }
                                }
                                Some(address) => {
                                    self.bytecodes.push(*address);
                                }
                            }
                        }
                        None => {}
                    }
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

        assert_eq!(compiler.bytecodes, vec![0x0]);
    }

    #[test]
    fn it_parses_brk_as_opcode_0() {
        let mut compiler = Compiler::new();
        compiler.parse("brk");

        assert_eq!(compiler.bytecodes, vec![0x0]);
    }

    #[test]
    fn it_parses_operand_a_as_hex() {
        let mut compiler = Compiler::new();
        compiler.parse("brk f");

        assert_eq!(compiler.bytecodes, vec![0x0, 0xf]);
    }

    #[test]
    fn it_parses_operand_a_as_a_label() {
        let mut compiler = Compiler::new();
        compiler.parse("brk end\nend:");

        assert_eq!(compiler.bytecodes, vec![0x0, 0x4]);
    }

    #[test]
    fn it_parses_operand_b_as_hex() {
        let mut compiler = Compiler::new();
        compiler.parse("brk f f");

        assert_eq!(compiler.bytecodes, vec![0x0, 0xf, 0xf]);
    }

    #[test]
    fn it_parses_operand_b_as_a_label() {
        let mut compiler = Compiler::new();
        compiler.parse("brk f end\nend:");

        assert_eq!(compiler.bytecodes, vec![0x0, 0xf, 0x4]);
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
    fn it_compiles_the_simple_ctrl_c_example() {
        let mut compiler = Compiler::new();

        compiler.parse(" f 2 0 3  ; Read key press and store it in M[2]\n 8 2 2 3  ; Subtract M[3] \
                    from M[2] and store the result in M[2]\n 2 b 2 f  ; If M[2] equals 0, then \
                    set PC to M[b]\n 1 e 0 0  ; Else, set PC to M[e] ");

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

        compiler.parse(" \
        x:\n \
          nop 0 0 0\n \
        \n \
        ctrl-c:\n \
          add 4 9 a\n \
          nop 3 0 0\n \
        \n \
        check-ctrl-c:\n \
          key x 0 0              ; Read key press and store it in M[x]\n \
          sub x x ctrl-c         ; Subtract M[ctrl-c] from M[x] and store the result in M[x]\n \
          beq 17 x exit          ; If M[x] equals 0, then set PC to M[17]\n \
          lpc 1a check-ctrl-c 0  ; Else, set PC to M[1a]\n \
        \n \
        exit:\n \
          brk 0 0 0 \
        ");

        let mut labels = HashMap::new();
        labels.insert("x".to_string(), 0);
        labels.insert("ctrl-c".to_string(), 4);
        labels.insert("check-ctrl-c".to_string(), 12);
        labels.insert("exit".to_string(), 28);

        assert_eq!(compiler.labels, labels);

        assert_eq!(compiler.bytecodes,
                   vec![0x10, 0x0, 0x0, 0x0, 0x7, 0x4, 0x9, 0xa, 0x10, 0x3, 0x0, 0x0, 0xf, 0x0,
                        0x0, 0x0, 0x8, 0x0, 0x0, 0x4, 0x2, 0x17, 0x0, 0x1c, 0x1, 0x1a, 0xc, 0x0,
                        0x0, 0x0, 0x0, 0x0]);
    }
}
