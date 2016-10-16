use termion;

use super::sixel;
use std::io::{Read, Write};
use std::vec::Vec;

pub struct Computer<'a> {
    memory: Vec<u32>,
    counter: u32,
    input: Option<&'a mut (Read + 'a)>,
    output: Option<&'a mut (Write + 'a)>,
}

impl<'a> Computer<'a> {
    /// Create a new `Computer`.
    ///
    /// The computer will start without any memory. The program counter will
    /// start at zero.
    ///
    /// # Example
    ///
    /// ```
    /// use chifir::computer::Computer;
    ///
    /// let computer = Computer::new();
    ///
    /// assert_eq!([0; 0], computer.dump());
    /// assert_eq!(0, computer.position());
    /// ```
    pub fn new() -> Self {
        Computer {
            memory: Vec::new(),
            counter: 0,
            input: None,
            output: None,
        }
    }

    pub fn input(&mut self, input: &'a mut Read) {
        self.input = Some(input);
    }

    pub fn output(&mut self, output: &'a mut Write) {
        self.output = Some(output);
    }

    /// Returns the current location of the program counter.
    ///
    /// # Examples
    ///
    /// ```
    /// use chifir::computer::Computer;
    ///
    /// let mut computer = Computer::new();
    ///
    /// computer.load(vec![
    ///     0xd, 0x0, 0x0, 0x0,  // drw
    ///     0x1, 0x6, 0x0, 0x0,  // lpc /2
    /// ]);
    ///
    /// assert_eq!(computer.position(), 0);
    ///
    /// computer.step();
    ///
    /// assert_eq!(computer.position(), 4);
    /// ```
    pub fn position(&self) -> u32 {
        self.counter
    }

    /// Returns the next opcode that will be executed.
    ///
    /// # Examples
    ///
    /// ```
    /// use chifir::computer::Computer;
    ///
    /// let mut computer = Computer::new();
    ///
    /// computer.load(vec![
    ///     0x1, 0x2, 0x0, 0x0,  // lpc /2
    /// ]);
    ///
    /// assert_eq!(computer.next(), 0x1);
    /// ```
    pub fn next(&mut self) -> u32 {
        let counter = self.counter;
        self.read(counter)
    }

    /// Copies the elements from `iter` into memory.
    ///
    /// The program counter will be reset to zero.
    ///
    /// # Example
    ///
    /// ```
    /// use chifir::computer::Computer;
    ///
    /// let mut computer = Computer::new();
    ///
    /// computer.load(vec![
    ///     1, 2, 4, 0
    /// ]);
    ///
    /// assert_eq!([1, 2, 4, 0], computer.dump());
    ///
    /// computer.step();
    /// computer.step();
    ///
    /// assert_eq!(4, computer.position());
    ///
    /// computer.load(vec![]);
    ///
    /// assert_eq!([0; 0], computer.dump());
    /// assert_eq!(0, computer.position());
    /// ```
    pub fn load<I: IntoIterator<Item = u32>>(&mut self, iter: I) {
        self.memory.clear();
        self.memory.extend(iter);
        self.counter = 0;
    }

    /// Copies the elements from `slice` into memory.
    ///
    /// The program counter will be reset to zero.
    ///
    /// # Example
    ///
    /// ```
    /// use chifir::computer::Computer;
    ///
    /// let mut computer = Computer::new();
    ///
    /// computer.load_from_slice(&[
    ///     1, 2, 4, 0
    /// ]);
    ///
    /// assert_eq!([1, 2, 4, 0], computer.dump());
    ///
    /// computer.step();
    /// computer.step();
    ///
    /// assert_eq!(4, computer.position());
    ///
    /// computer.load(vec![]);
    ///
    /// assert_eq!([0; 0], computer.dump());
    /// assert_eq!(0, computer.position());
    /// ```
    pub fn load_from_slice(&mut self, slice: &[u32]) {
        self.memory.clear();
        self.memory.extend_from_slice(slice);
        self.counter = 0;
    }

    /// Extracts a slice containing the contents of memory.
    ///
    /// # Example
    ///
    /// ```
    /// use chifir::computer::Computer;
    ///
    /// let mut computer = Computer::new();
    ///
    /// computer.load_from_slice(&[
    ///     1, 2, 3, 4
    /// ]);
    ///
    /// assert_eq!([1, 2, 3, 4], computer.dump());
    /// ```
    pub fn dump(&self) -> &[u32] {
        self.memory.as_slice()
    }

    /// Executes the next instruction.
    ///
    /// # Examples
    ///
    /// ```
    /// use chifir::computer::Computer;
    ///
    /// let mut computer = Computer::new();
    ///
    /// computer.load(vec![
    ///     0x1, 0x3, 0x0, 0x8,  // lpc /3 0 8
    ///     0x0, 0x0, 0x0, 0x0,  // brk
    ///     0x2, 0xa, 0x4, 0x0,  // beq /2 4
    /// ]);
    ///
    /// assert_eq!(computer.next(), 0x1);
    ///
    /// computer.step();
    /// computer.step();
    ///
    /// assert_eq!(computer.next(), 0x0);
    /// ```
    pub fn step(&mut self) {
        let counter = self.counter;
        let opcode = self.read(counter);
        let a = self.read(counter + 1);
        let b = self.read(counter + 2);
        let c = self.read(counter + 3);
        self.exec(opcode, a, b, c);
    }

    fn read(&mut self, index: u32) -> u32 {
        let index = index as usize;

        if index >= self.memory.len() {
            self.memory.resize(index + 1, 0);
        }

        return self.memory[index];
    }

    fn write(&mut self, index: u32, value: u32) {
        let index = index as usize;

        if index >= self.memory.len() {
            self.memory.resize(index + 1, 0);
        }

        self.memory[index] = value;
    }

    fn render(&mut self) {
        let width = 512;
        let height = 684;
        let start = 1_048_576;
        let end = start + (width * height);
        self.read(start);
        self.read(end);

        let width = width as usize;
        let height = height as usize;
        let start = start as usize;
        let end = end as usize;
        let memory = &self.memory[start..end];

        match self.output {
            Some(ref mut output) => {
                write!(output,
                       "{}{}{}{}",
                       termion::cursor::Goto(1, 1),
                       sixel::begin(),
                       sixel::from(memory, width, height, true),
                       sixel::end())
                    .unwrap();
                output.flush().unwrap();
            }
            None => {}
        }
    }

    fn exec(&mut self, opcode: u32, a: u32, b: u32, c: u32) {
        match opcode {
            // Halt execution
            0 => {}

            // PC <- M[A]
            1 => {
                self.counter = self.read(a);
            }

            // If M[B] = 0, then PC <- M[A]
            2 => {
                if 0 == self.read(b) {
                    self.counter = self.read(a);
                } else {
                    self.counter += 4;
                }
            }

            // M[A] <- PC
            3 => {
                let counter = self.counter;
                self.write(a, counter);
                self.counter += 4;
            }

            // M[A] <- M[B]
            4 => {
                let b = self.read(b);
                self.write(a, b);
                self.counter += 4;
            }

            // M[A] <- M[M[B]]
            5 => {
                let b = self.read(b);
                let b = self.read(b);
                self.write(a, b);
                self.counter += 4;
            }

            // M[M[B]] <- M[A]
            6 => {
                let a = self.read(a);
                let b = self.read(b);
                self.write(b, a);
                self.counter += 4;
            }

            // M[A] <- M[B] + M[C]
            7 => {
                let b = self.read(b);
                let c = self.read(c);
                self.write(a, b.wrapping_add(c));
                self.counter += 4;
            }

            // M[A] <- M[B] - M[C]
            8 => {
                let b = self.read(b);
                let c = self.read(c);
                self.write(a, b.wrapping_sub(c));
                self.counter += 4;
            }

            // M[A] <- M[B] * M[C]
            9 => {
                let b = self.read(b);
                let c = self.read(c);
                self.write(a, b.wrapping_mul(c));
                self.counter += 4;
            }

            // M[A] <- M[B] / M[C]
            10 => {
                let b = self.read(b);
                let c = self.read(c);
                if c > 0 {
                    self.write(a, b / c);
                } else {
                    self.write(a, 0);
                }
                self.counter += 4;
            }

            // M[A] <- M[B] % M[C]
            11 => {
                let b = self.read(b);
                let c = self.read(c);
                if c > 0 {
                    self.write(a, b % c);
                } else {
                    self.write(a, 0);
                }
                self.counter += 4;
            }

            // If M[B] < M[C], then M[A] <- 1, else M[A] <- 0
            12 => {
                let b = self.read(b);
                let c = self.read(c);
                if b < c {
                    self.write(a, 1);
                } else {
                    self.write(a, 0);
                }
                self.counter += 4;
            }

            // MA[A] <- NOT(M[B] AND M[C])
            13 => {
                let b = self.read(b);
                let c = self.read(c);
                self.write(a, !(b & c));
                self.counter += 4;
            }

            // Refresh the screen
            14 => {
                self.render();
                self.counter += 4;
            }

            // Get one character from the keyboard and store it into M[A]
            15 => {
                let mut bytes = Vec::new();
                let mut result: Option<u32> = None;

                match self.input {
                    Some(ref mut input) => {
                        match input.read_to_end(&mut bytes) {
                            Ok(size) => {
                                if size > 0 {
                                    result = Some(bytes[size - 1] as u32)
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }

                match result {
                    Some(byte) => {
                        self.write(a, byte);
                        self.counter += 4;
                    }
                    _ => {}
                }
            }

            // Skip this instruction
            16 => {
                self.counter += 4;
            }

            // Unknown opcode
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Computer;
    use std::io::{Read, Seek, SeekFrom, Cursor};

    #[test]
    fn it_runs_opcode_0() {
        // Halt execution
        let mut m = Computer::new();
        m.load_from_slice(&[0]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!(0, m.position());
    }

    #[test]
    fn it_runs_opcode_1() {
        // PC <- M[A]
        let mut m = Computer::new();
        m.load_from_slice(&[1, 4, 0, 0, 2]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!(2, m.position());
    }

    #[test]
    fn it_runs_opcode_2_then_branch() {
        // If M[B] = 0, then PC <- M[A]
        let mut m = Computer::new();
        m.load_from_slice(&[2, 4, 5, 0, 2, 0]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!(2, m.position());
    }

    #[test]
    fn it_runs_opcode_2_else_branch() {
        // If M[B] = 0, then PC <- M[A]
        let mut m = Computer::new();
        m.load_from_slice(&[2, 4, 5, 0, 2, 1]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_runs_opcode_3() {
        // M[A] <- PC
        let mut m = Computer::new();
        m.load_from_slice(&[3, 4, 0, 0, 1]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!([3, 4, 0, 0, 0], m.dump());
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_runs_opcode_4() {
        // M[A] <- M[B]
        let mut m = Computer::new();
        m.load_from_slice(&[4, 4, 5, 0, 6, 7]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!([4, 4, 5, 0, 7, 7], m.dump());
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_runs_opcode_5() {
        // M[A] <- M[M[B]]
        let mut m = Computer::new();
        m.load_from_slice(&[5, 4, 5, 0, 6, 7, 0, 8]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!([5, 4, 5, 0, 8, 7, 0, 8], m.dump());
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_runs_opcode_6() {
        // M[M[B]] <- M[A]
        let mut m = Computer::new();
        m.load_from_slice(&[6, 4, 5, 0, 8, 6, 7]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!([6, 4, 5, 0, 8, 6, 8], m.dump());
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_runs_opcode_7() {
        // M[A] <- M[B] + M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[7, 4, 5, 6, 0, 11, 2]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!([7, 4, 5, 6, 13, 11, 2], m.dump());
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_prevents_overflow_when_running_opcode_7() {
        // M[A] <- M[B] + M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[7, 4, 5, 6, 1, u32::max_value(), 1]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!([7, 4, 5, 6, 0, u32::max_value(), 1], m.dump());
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_runs_opcode_8() {
        // M[A] <- M[B] - M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[8, 4, 5, 6, 0, 11, 2]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!([8, 4, 5, 6, 9, 11, 2], m.dump());
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_prevents_overflow_while_runing_opcode_8() {
        // M[A] <- M[B] - M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[8, 4, 5, 6, 1, 2, 11]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!([8, 4, 5, 6, 4294967287, 2, 11], m.dump());
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_runs_opcode_9() {
        // M[A] <- M[B] * M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[9, 4, 5, 6, 0, 11, 2]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!([9, 4, 5, 6, 22, 11, 2], m.dump());
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_prevents_overflow_when_running_opcode_9() {
        // M[A] <- M[B] * M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[9, 4, 5, 6, 0, u32::max_value(), 2]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!([9, 4, 5, 6, 4294967294, u32::max_value(), 2], m.dump());
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_runs_opcode_10() {
        // M[A] <- M[B] / M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[10, 4, 5, 6, 0, 11, 2]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!([10, 4, 5, 6, 5, 11, 2], m.dump());
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_prevents_divide_by_zero_errors_when_running_opcode_10() {
        // M[A] <- M[B] / M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[10, 4, 5, 6, 1, 11, 0]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!([10, 4, 5, 6, 0, 11, 0], m.dump());
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_runs_opcode_11() {
        // M[A] <- M[B] % M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[11, 4, 5, 6, 0, 11, 2]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!([11, 4, 5, 6, 1, 11, 2], m.dump());
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_prevents_divide_by_zero_errors_when_running_opcode_11() {
        // M[A] <- M[B] % M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[11, 4, 5, 6, 1, 11, 0]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!([11, 4, 5, 6, 0, 11, 0], m.dump());
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_runs_opcode_12_then_branch() {
        // If M[B] < M[C], then M[A] <- 1, else M[A] <- 0
        let mut m = Computer::new();
        m.load_from_slice(&[12, 4, 5, 6, 2, 8, 9]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!([12, 4, 5, 6, 1, 8, 9], m.dump());
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_runs_opcode_12_else_branch() {
        // If M[B] < M[C], then M[A] <- 1, else M[A] <- 0
        let mut m = Computer::new();
        m.load_from_slice(&[12, 4, 5, 6, 2, 9, 8]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!([12, 4, 5, 6, 0, 9, 8], m.dump());
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_runs_opcode_13() {
        // M[A] <- NOT(M[B] AND M[C])
        let mut m = Computer::new();
        m.load_from_slice(&[13, 4, 5, 6, 0, 0xfffffffe, 0xfffffffd]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!([13, 4, 5, 6, 0x3, 0xfffffffe, 0xfffffffd], m.dump());
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_runs_opcode_14() {
        // Refresh the screen
        let mut output = Cursor::new(Vec::new());

        {
            let mut m = Computer::new();
            m.load_from_slice(&[14, 0, 0, 0]);
            m.output(&mut output);

            // Move the program counter after rendering.
            assert_eq!(0, m.position());
            m.step();
            assert_eq!(4, m.position());
        }

        let mut buffer: Vec<u8> = Vec::new();
        output.seek(SeekFrom::Start(0)).unwrap();
        output.read_to_end(&mut buffer).unwrap();

        // Move the cursor to (1,1).
        assert_eq!(&buffer[0..6], &[27, 91, 49, 59, 49, 72]);

        // Put the terminal in Sixel graphics mode.
        assert_eq!(&buffer[6..9], &[27, 80, 113]);

        // Return the terminal to normal mode.
        assert_eq!(&buffer[buffer.len() - 2..buffer.len()], &[27, 92]);
    }

    #[test]
    fn it_runs_opcode_15_blocking() {
        // Get one character from the keyboard and store it into M[A]
        let mut m = Computer::new();
        m.load_from_slice(&[15, 1, 0, 0]);

        // Don't move the program counter if reading failed.
        assert_eq!(0, m.position());
        m.step();
        assert_eq!(0, m.position());
    }

    #[test]
    fn it_runs_opcode_15_non_blocking() {
        // Get one character from the keyboard and store it into M[A]
        let mut input = Cursor::new(vec![8, 10, 13, 32]);

        let mut m = Computer::new();
        m.input(&mut input);
        m.load_from_slice(&[15, 1, 0, 0]);

        // Move the program counter after reading.
        assert_eq!(0, m.position());
        m.step();
        assert_eq!(4, m.position());

        // Only save the last key pressed.
        assert_eq!([15, 32, 0, 0], m.dump());
    }

    #[test]
    fn it_runs_opcode_16() {
        // Skip this instruction
        let mut m = Computer::new();
        m.load_from_slice(&[16, 0, 0, 0]);

        assert_eq!(0, m.position());
        m.step();
        assert_eq!(4, m.position());
    }

    #[test]
    fn it_provides_safe_memory_access_when_stepping() {
        let mut m = Computer::new();
        m.load_from_slice(&[1, 2, 4, 0]);

        m.step();
        m.step();
        assert_eq!([1, 2, 4, 0, 0, 0, 0, 0], m.dump());
    }
}
