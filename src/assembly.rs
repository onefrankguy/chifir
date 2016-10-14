use pest::prelude::*;

use std::vec::Vec;
use std::string::String;

impl_rdp! {
    grammar! {
        program = { instruction ~ (["\n"] ~ instruction)* }
        instruction = { opcode ~ operand ~ operand ~ operand }
        opcode = _{ number }
        operand = _{ number }

        number = @{ (['0'..'9'] | ['a'..'f'] | ['A'..'F'])+ }

        whitespace = _{ [" "] }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use pest::*;

    #[test]
    fn it_parses_programs_as_machine_code() {
        let mut parser = Rdp::new(StringInput::new("0 a b c\n1 2 3 4"));

        assert!(parser.program());
        assert!(parser.end());

        let tokens = vec![
            Token::new(Rule::program, 0, 15),
            Token::new(Rule::instruction, 0, 7),
            Token::new(Rule::number, 0, 1),
            Token::new(Rule::number, 2, 3),
            Token::new(Rule::number, 4, 5),
            Token::new(Rule::number, 6, 7),
            Token::new(Rule::instruction, 8, 15),
            Token::new(Rule::number, 8, 9),
            Token::new(Rule::number, 10, 11),
            Token::new(Rule::number, 12, 13),
            Token::new(Rule::number, 14, 15),
        ];

        assert_eq!(parser.queue(), &tokens);
    }
}
