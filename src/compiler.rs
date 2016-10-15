use std::vec::Vec;
use std::string::String;

struct Compiler {
    pub lines: Vec<String>,
    pub instructions: Vec<String>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            lines: Vec::new(),
            instructions: Vec::new(),
        }
    }

    pub fn parse(&mut self, assembly: &str) {
        self.split_lines(assembly);
        self.strip_comments();
    }

    fn strip_comments(&mut self) {
        let mut lines = self.lines.iter();
        while let Some(line) = lines.next() {
            let trimmed_line = line.trim();
            if !trimmed_line.is_empty() && !trimmed_line.starts_with(";") {
                let mut instruction = String::new();
                let stripped_instruction = match trimmed_line.find(';') {
                    Some(index) => trimmed_line.split_at(index).0,
                    None => trimmed_line,
                };
                instruction.push_str(stripped_instruction.trim());
                self.instructions.push(instruction);
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
}
