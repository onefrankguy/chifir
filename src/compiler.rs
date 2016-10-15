use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;

struct Compiler {
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
                    match bytecodes.next() {
                        Some("brk") => {
                            self.bytecodes.push(0);
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
        compiler.parse("0 0 0 0");

        assert_eq!(compiler.bytecodes, vec![0]);
    }

    #[test]
    fn it_parses_brk_as_opcode_0() {
        let mut compiler = Compiler::new();
        compiler.parse("brk 0 0 0");

        assert_eq!(compiler.bytecodes, vec![0]);
    }
}
