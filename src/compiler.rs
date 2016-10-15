use std::vec::Vec;
use std::string::String;

struct Compiler {
    pub lines: Vec<String>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { lines: Vec::new() }
    }

    pub fn parse(&mut self, assembly: &str) {
        let mut line = String::new();
        let mut chars = assembly.chars();

        while let Some(c) = chars.next() {
            match c {
                // Line Feed
                // Vertical Tab
                // Form Feed
                '\u{000A}' | '\u{000B}' | '\u{000C}' => {
                    self.lines.push(line);
                    line = String::new();
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
    fn it_ignors_trailing_separators() {
        let mut compiler = Compiler::new();
        compiler.parse("0 0 0 0\n0 0 0 0\n");

        assert_eq!(compiler.lines.len(), 2);
    }
}
