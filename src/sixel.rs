pub fn begin() -> String {
    "\x1bPq".to_string()
}

pub fn end() -> String {
    "\x1b\\".to_string()
}

pub fn from(memory: &[u32], width: usize, height: usize, border: bool) -> String {
    let mut pixels: Vec<u8> = Vec::new();
    let mut row = 0;

    if border {
        for _ in 0..(width + 2) {
            pixels.push(95);
        }
        pixels.push(36);
        pixels.push(45);
    }

    while row < height {
        if border {
            pixels.push(126);
        }

        for x in 0..width {
            let mut byte: u8 = 0;

            for y in 0..6 {
                let offset = x + ((row + y) * width);
                if offset < memory.len() {
                    if memory[offset] > 0 {
                        byte = byte | (1 << y);
                    }
                }
            }

            pixels.push(byte + 63);
        }

        if border {
            pixels.push(126);
        }

        // Push "$-" to move to the next row.
        pixels.push(36);
        pixels.push(45);
        row += 6;
    }

    if border {
        for _ in 0..(width + 2) {
            pixels.push(64);
        }
        pixels.push(36);
        pixels.push(45);
    }

    String::from_utf8(pixels).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_converts_000000_to_question_mark() {
        assert_eq!(super::from(&[0, 0, 0, 0, 0, 0], 1, 6, false), "?$-");
    }

    #[test]
    fn it_converts_000111_to_w() {
        assert_eq!(super::from(&[0, 0, 0, 1, 1, 1], 1, 6, false), "w$-");
    }

    #[test]
    fn it_converts_111000_to_capital_f() {
        assert_eq!(super::from(&[1, 1, 1, 0, 0, 0], 1, 6, false), "F$-");
    }

    #[test]
    fn it_converts_111111_to_tilde() {
        assert_eq!(super::from(&[1, 1, 1, 1, 1, 1], 1, 6, false), "~$-");
    }

    #[test]
    fn it_displays_capital_a() {
        assert_eq!(super::from(&[0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0,
                                 0, 0, 0],
                               4,
                               6,
                               false),
                   "]DD]$-");
    }
}
