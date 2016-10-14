use pest::prelude::*;

use std::vec::Vec;
use std::string::String;
use std::collections::LinkedList;

impl_rdp! {
    grammar! {
        program = _{ instruction ~ (["\n"] ~ instruction)* }
        instruction = _{ opcode ~ operand ~ operand ~ operand }
        opcode = _{ brk | number }
        operand = _{ number }

        brk = { [i"brk"] }
        number = @{ (['0'..'9'] | ['a'..'f'] | ['A'..'F'])+ }

        whitespace = _{ [" "] }
    }

    process! {
        compile(&self) -> Vec<u32> {
            (list: _numbers()) => {
                let mut instructions = Vec::new();
                instructions.extend(list.iter());
                instructions
            }
        }

        _numbers(&self) -> LinkedList<u32> {
            (_: brk, mut tail: _numbers()) => {
                tail.push_front(0x0);
                tail
            },
            (&head: number, mut tail: _numbers()) => {
                tail.push_front(u32::from_str_radix(head, 16).unwrap());
                tail
            },
            () => {
                LinkedList::new()
            }
        }
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
            Token::new(Rule::number, 0, 1),
            Token::new(Rule::number, 2, 3),
            Token::new(Rule::number, 4, 5),
            Token::new(Rule::number, 6, 7),
            Token::new(Rule::number, 8, 9),
            Token::new(Rule::number, 10, 11),
            Token::new(Rule::number, 12, 13),
            Token::new(Rule::number, 14, 15),
        ];

        assert_eq!(parser.queue(), &tokens);
        assert_eq!(parser.compile(),
                   vec![0x0, 0xa, 0xb, 0xc, 0x1, 0x2, 0x3, 0x4]);
    }

    #[test]
    fn it_parses_opcode_brk() {
        let mut parser = Rdp::new(StringInput::new("brk a b c"));

        assert!(parser.program());
        assert!(parser.end());

        let tokens = vec![
            Token::new(Rule::brk, 0, 3),
            Token::new(Rule::number, 4, 5),
            Token::new(Rule::number, 6, 7),
            Token::new(Rule::number, 8, 9),
        ];

        assert_eq!(parser.queue(), &tokens);
        assert_eq!(parser.compile(), vec![0x0, 0xa, 0xb, 0xc]);
    }
}
