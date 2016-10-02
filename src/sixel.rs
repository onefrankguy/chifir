pub fn clear() -> String {
    "\x1b[2J".to_string()
}

pub fn begin() -> String {
    "\x1bPq".to_string()
}

pub fn end() -> String {
    "\x1b\\".to_string()
}

pub fn from(memory: &[u32], width: usize, height: usize) -> String {
    let mut pixels: Vec<u8> = Vec::new();
    let mut row = 0;

    while row < height {
        for x in 0..width {
            let mut byte: u8 = 0;

            for y in 0..6 {
                let offset = x + (y * width);
                if memory[offset] > 0 {
                    byte = byte | (1 << y);
                }
            }

            pixels.push(byte + 63);
        }

        row += 6;
    }

    String::from_utf8(pixels).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_converts_000000_to_question_mark() {
        assert_eq!(super::from(&[0, 0, 0, 0, 0, 0], 1, 6), "?");
    }

    #[test]
    fn it_converts_000111_to_w() {
        assert_eq!(super::from(&[0, 0, 0, 1, 1, 1], 1, 6), "w");
    }

    #[test]
    fn it_converts_111000_to_capital_f() {
        assert_eq!(super::from(&[1, 1, 1, 0, 0, 0], 1, 6), "F");
    }

    #[test]
    fn it_converts_111111_to_tilde() {
        assert_eq!(super::from(&[1, 1, 1, 1, 1, 1], 1, 6), "~");
    }

    #[test]
    fn it_displays_capital_a() {
        assert_eq!(super::from(&[0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0,
                                 0, 0, 0],
                               4,
                               6),
                   "]DD]");
    }
}
