//! A virtual computer for executing bytecode.

use termion;

use super::sixel;
use std::io::{self, Read, Write, Cursor};
use std::marker::Send;
use std::vec::Vec;

pub struct Computer {
    memory: Vec<u32>,
    counter: u32,
    input: Option<Box<Read + Send>>,
    output: Option<Box<Write + Send>>,
    keyboard: Option<u8>,
    display_address: u32,
    display_width: u32,
    display_height: u32,
    display: Vec<u8>,
    read_position: usize,
}

impl Computer {
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
    /// assert_eq!(0, computer.next());
    /// ```
    pub fn new() -> Self {
        Computer {
            memory: Vec::new(),
            counter: 0,
            input: None,
            output: None,
            keyboard: None,
            display_address: 1_048_576,
            display_width: 512,
            display_height: 684,
            display: Vec::new(),
            read_position: 0,
        }
    }

    /// Binds a reader for getting keyboard input.
    ///
    /// Only the last pressed key is used. An in memory keyboard is provided
    /// through the `Write` trait.
    ///
    /// # Examples
    ///
    /// A `Computer` can be constructed with an input reader.
    ///
    /// ```
    /// use chifir::computer::Computer;
    /// use std::io::Cursor;
    ///
    /// let stdin = Box::new(Cursor::new(vec![0x8, 0x71]));
    ///
    /// let mut computer  = Computer::new().input(stdin);
    /// computer.load(vec![
    ///     0xf, 0x2, 0x0, 0x0
    /// ]);
    ///
    /// computer.step();
    ///
    /// assert_eq!([0xf, 0x2, 0x71, 0x0], computer.dump());
    /// ```
    ///
    /// The in memory keyboard can also be used directly.
    ///
    /// ```
    /// use chifir::computer::Computer;
    /// use std::io::Write;
    ///
    /// let mut computer = Computer::new();
    /// computer.load(vec![
    ///     0xf, 0x2, 0x0, 0x0
    /// ]);
    ///
    /// let input = &[0x8, 0x71];
    ///
    /// computer.write(input).unwrap();
    /// computer.step();
    ///
    /// assert_eq!([0xf, 0x2, 0x71, 0x0], computer.dump());
    /// ```
    pub fn input(mut self, input: Box<Read + Send>) -> Self {
        self.input = Some(input);
        self
    }

    /// Binds a writer for getting display output.
    ///
    /// An in memory display is provided through the `Read` trait.
    ///
    /// # Examples
    ///
    /// A `Computer` can be constructed with an output writer.
    ///
    /// ```
    /// use chifir::computer::Computer;
    /// use std::io::{Read, Cursor};
    ///
    /// let stdout = Box::new(Cursor::new(Vec::new()));
    ///
    /// let mut computer  = Computer::new().output(stdout);
    /// computer.load(vec![
    ///     14, 0, 0, 0
    /// ]);
    ///
    /// computer.step();
    ///
    /// let mut output = Vec::new();
    ///
    /// computer.read_to_end(&mut output).unwrap();
    ///
    /// // Cursor is moved to (1,1) when rendering output
    /// let move_cursor: [u8; 6] = [27, 91, 49, 59, 49, 72];
    ///
    /// assert_eq!(move_cursor, output[0..6]);
    /// ```
    ///
    /// The in memory display is populated even when no writer is connected.
    ///
    /// ```
    /// use chifir::computer::Computer;
    /// use std::io::Read;
    ///
    /// let mut computer = Computer::new();
    /// computer.load(vec![
    ///     14, 0, 0, 0
    /// ]);
    ///
    /// computer.step();
    ///
    /// let mut output = Vec::new();
    ///
    /// computer.read_to_end(&mut output).unwrap();
    ///
    /// // Cursor is moved to (1,1) when rendering output
    /// let move_cursor: [u8; 6] = [27, 91, 49, 59, 49, 72];
    ///
    /// assert_eq!(move_cursor, output[0..6]);
    /// ```
    pub fn output(mut self, output: Box<Write + Send>) -> Self {
        self.output = Some(output);
        self
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
    pub fn next(&self) -> u32 {
        *self.memory.get(self.counter as usize).unwrap_or(&0)
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
    ///     1, 2, 4, 0,  // PC <- M[2]
    ///     2, 7, 3, 0,  // If M[3] = 0, then PC <- M[7]
    /// ]);
    ///
    /// computer.step();
    /// assert_eq!(2, computer.next());
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
    ///     1, 2, 4, 0,  // PC <- M[2]
    ///     2, 7, 3, 0,  // If M[3] = 0, then PC <- M[7]
    /// ]);
    ///
    /// computer.step();
    /// assert_eq!(2, computer.next());
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
        let opcode = self.fetch(counter);
        let a = self.fetch(counter + 1);
        let b = self.fetch(counter + 2);
        let c = self.fetch(counter + 3);
        self.exec(opcode, a, b, c);
    }

    fn fetch(&mut self, index: u32) -> u32 {
        let index = index as usize;

        if index >= self.memory.len() {
            self.memory.resize(index + 1, 0);
        }

        return self.memory[index];
    }

    fn store(&mut self, index: u32, value: u32) {
        let index = index as usize;

        if index >= self.memory.len() {
            self.memory.resize(index + 1, 0);
        }

        self.memory[index] = value;
    }

    fn render(&mut self) {
        let width = self.display_width;
        let height = self.display_height;
        let start = self.display_address;
        let end = start + (width * height);
        self.fetch(start);
        self.fetch(end);

        let width = width as usize;
        let height = height as usize;
        let start = start as usize;
        let end = end as usize;
        let memory = &self.memory[start..end];

        let mut buffer = Cursor::new(Vec::new());
        write!(buffer,
               "{}{}{}{}",
               termion::cursor::Goto(1, 1),
               sixel::begin(),
               sixel::from(memory, width, height, false),
               sixel::end())
            .unwrap();
        buffer.flush().unwrap();
        self.display = buffer.into_inner();
        self.read_position = 0;

        match self.output {
            Some(ref mut output) => {
                output.write(self.display.as_slice()).unwrap();
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
                self.counter = self.fetch(a);
            }

            // If M[B] = 0, then PC <- M[A]
            2 => {
                if 0 == self.fetch(b) {
                    self.counter = self.fetch(a);
                } else {
                    self.counter += 4;
                }
            }

            // M[A] <- PC
            3 => {
                let counter = self.counter;
                self.store(a, counter);
                self.counter += 4;
            }

            // M[A] <- M[B]
            4 => {
                let b = self.fetch(b);
                self.store(a, b);
                self.counter += 4;
            }

            // M[A] <- M[M[B]]
            5 => {
                let b = self.fetch(b);
                let b = self.fetch(b);
                self.store(a, b);
                self.counter += 4;
            }

            // M[M[B]] <- M[A]
            6 => {
                let a = self.fetch(a);
                let b = self.fetch(b);
                self.store(b, a);
                self.counter += 4;
            }

            // M[A] <- M[B] + M[C]
            7 => {
                let b = self.fetch(b);
                let c = self.fetch(c);
                self.store(a, b.wrapping_add(c));
                self.counter += 4;
            }

            // M[A] <- M[B] - M[C]
            8 => {
                let b = self.fetch(b);
                let c = self.fetch(c);
                self.store(a, b.wrapping_sub(c));
                self.counter += 4;
            }

            // M[A] <- M[B] * M[C]
            9 => {
                let b = self.fetch(b);
                let c = self.fetch(c);
                self.store(a, b.wrapping_mul(c));
                self.counter += 4;
            }

            // M[A] <- M[B] / M[C]
            10 => {
                let b = self.fetch(b);
                let c = self.fetch(c);
                if c > 0 {
                    self.store(a, b / c);
                } else {
                    self.store(a, 0);
                }
                self.counter += 4;
            }

            // M[A] <- M[B] % M[C]
            11 => {
                let b = self.fetch(b);
                let c = self.fetch(c);
                if c > 0 {
                    self.store(a, b % c);
                } else {
                    self.store(a, 0);
                }
                self.counter += 4;
            }

            // If M[B] < M[C], then M[A] <- 1, else M[A] <- 0
            12 => {
                let b = self.fetch(b);
                let c = self.fetch(c);
                if b < c {
                    self.store(a, 1);
                } else {
                    self.store(a, 0);
                }
                self.counter += 4;
            }

            // MA[A] <- NOT(M[B] AND M[C])
            13 => {
                let b = self.fetch(b);
                let c = self.fetch(c);
                self.store(a, !(b & c));
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
                let mut result = self.keyboard;

                match self.input {
                    Some(ref mut input) => {
                        match input.read_to_end(&mut bytes) {
                            Ok(size) => {
                                if size > 0 {
                                    result = Some(bytes[size - 1])
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }

                match result {
                    Some(byte) => {
                        self.store(a, byte as u32);
                        self.counter += 4;
                    }
                    _ => {}
                }
            }

            // Skip this instruction
            16 => {
                self.counter += 4;
            }

            // Configure display at M[A] with width B and height C
            17 => {
                self.display_address = a;
                self.display_width = b;
                self.display_height = c;
                self.counter += 4;
            }

            // Unknown opcode
            _ => {}
        }
    }
}

impl Write for Computer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match buf.last() {
            Some(byte) => {
                self.keyboard = Some(*byte);
                Ok(buf.len())
            }
            None => {
                self.keyboard = None;
                Ok(0)
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Read for Computer {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        use std::cmp;

        let amt = cmp::min(self.display.len() - self.read_position, buf.len());
        let a = &self.display[self.read_position..(self.read_position + amt)];
        buf[..amt].copy_from_slice(a);
        self.read_position += amt;
        Ok(amt)
    }
}

#[cfg(test)]
mod tests {
    use super::Computer;
    use std::io::{Read, Cursor};

    #[test]
    fn it_runs_opcode_0() {
        // Halt execution
        let mut m = Computer::new();
        m.load_from_slice(&[0]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!(0, m.counter);
    }

    #[test]
    fn it_runs_opcode_1() {
        // PC <- M[A]
        let mut m = Computer::new();
        m.load_from_slice(&[1, 4, 0, 0, 2]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!(2, m.counter);
    }

    #[test]
    fn it_runs_opcode_2_then_branch() {
        // If M[B] = 0, then PC <- M[A]
        let mut m = Computer::new();
        m.load_from_slice(&[2, 4, 5, 0, 2, 0]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!(2, m.counter);
    }

    #[test]
    fn it_runs_opcode_2_else_branch() {
        // If M[B] = 0, then PC <- M[A]
        let mut m = Computer::new();
        m.load_from_slice(&[2, 4, 5, 0, 2, 1]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_runs_opcode_3() {
        // M[A] <- PC
        let mut m = Computer::new();
        m.load_from_slice(&[3, 4, 0, 0, 1]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!([3, 4, 0, 0, 0], m.dump());
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_runs_opcode_4() {
        // M[A] <- M[B]
        let mut m = Computer::new();
        m.load_from_slice(&[4, 4, 5, 0, 6, 7]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!([4, 4, 5, 0, 7, 7], m.dump());
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_runs_opcode_5() {
        // M[A] <- M[M[B]]
        let mut m = Computer::new();
        m.load_from_slice(&[5, 4, 5, 0, 6, 7, 0, 8]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!([5, 4, 5, 0, 8, 7, 0, 8], m.dump());
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_runs_opcode_6() {
        // M[M[B]] <- M[A]
        let mut m = Computer::new();
        m.load_from_slice(&[6, 4, 5, 0, 8, 6, 7]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!([6, 4, 5, 0, 8, 6, 8], m.dump());
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_runs_opcode_7() {
        // M[A] <- M[B] + M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[7, 4, 5, 6, 0, 11, 2]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!([7, 4, 5, 6, 13, 11, 2], m.dump());
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_prevents_overflow_when_running_opcode_7() {
        // M[A] <- M[B] + M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[7, 4, 5, 6, 1, u32::max_value(), 1]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!([7, 4, 5, 6, 0, u32::max_value(), 1], m.dump());
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_runs_opcode_8() {
        // M[A] <- M[B] - M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[8, 4, 5, 6, 0, 11, 2]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!([8, 4, 5, 6, 9, 11, 2], m.dump());
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_prevents_overflow_while_runing_opcode_8() {
        // M[A] <- M[B] - M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[8, 4, 5, 6, 1, 2, 11]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!([8, 4, 5, 6, 4294967287, 2, 11], m.dump());
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_runs_opcode_9() {
        // M[A] <- M[B] * M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[9, 4, 5, 6, 0, 11, 2]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!([9, 4, 5, 6, 22, 11, 2], m.dump());
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_prevents_overflow_when_running_opcode_9() {
        // M[A] <- M[B] * M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[9, 4, 5, 6, 0, u32::max_value(), 2]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!([9, 4, 5, 6, 4294967294, u32::max_value(), 2], m.dump());
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_runs_opcode_10() {
        // M[A] <- M[B] / M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[10, 4, 5, 6, 0, 11, 2]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!([10, 4, 5, 6, 5, 11, 2], m.dump());
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_prevents_divide_by_zero_errors_when_running_opcode_10() {
        // M[A] <- M[B] / M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[10, 4, 5, 6, 1, 11, 0]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!([10, 4, 5, 6, 0, 11, 0], m.dump());
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_runs_opcode_11() {
        // M[A] <- M[B] % M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[11, 4, 5, 6, 0, 11, 2]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!([11, 4, 5, 6, 1, 11, 2], m.dump());
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_prevents_divide_by_zero_errors_when_running_opcode_11() {
        // M[A] <- M[B] % M[C]
        let mut m = Computer::new();
        m.load_from_slice(&[11, 4, 5, 6, 1, 11, 0]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!([11, 4, 5, 6, 0, 11, 0], m.dump());
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_runs_opcode_12_then_branch() {
        // If M[B] < M[C], then M[A] <- 1, else M[A] <- 0
        let mut m = Computer::new();
        m.load_from_slice(&[12, 4, 5, 6, 2, 8, 9]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!([12, 4, 5, 6, 1, 8, 9], m.dump());
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_runs_opcode_12_else_branch() {
        // If M[B] < M[C], then M[A] <- 1, else M[A] <- 0
        let mut m = Computer::new();
        m.load_from_slice(&[12, 4, 5, 6, 2, 9, 8]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!([12, 4, 5, 6, 0, 9, 8], m.dump());
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_runs_opcode_13() {
        // M[A] <- NOT(M[B] AND M[C])
        let mut m = Computer::new();
        m.load_from_slice(&[13, 4, 5, 6, 0, 0xfffffffe, 0xfffffffd]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!([13, 4, 5, 6, 0x3, 0xfffffffe, 0xfffffffd], m.dump());
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_runs_opcode_14() {
        // Refresh the screen
        let mut m = Computer::new();
        m.load_from_slice(&[14, 0, 0, 0]);

        // Move the program counter after rendering.
        assert_eq!(0, m.counter);
        m.step();
        assert_eq!(4, m.counter);

        let mut buffer = Vec::new();
        m.read_to_end(&mut buffer).unwrap();

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
        assert_eq!(0, m.counter);
        m.step();
        assert_eq!(0, m.counter);
    }

    #[test]
    fn it_runs_opcode_15_non_blocking() {
        // Get one character from the keyboard and store it into M[A]
        let input = Box::new(Cursor::new(vec![8, 10, 13, 32]));

        let mut m = Computer::new().input(input);
        m.load_from_slice(&[15, 1, 0, 0]);

        // Move the program counter after reading.
        assert_eq!(0, m.counter);
        m.step();
        assert_eq!(4, m.counter);

        // Only save the last key pressed.
        assert_eq!([15, 32, 0, 0], m.dump());
    }

    #[test]
    fn it_runs_opcode_16() {
        // Skip this instruction
        let mut m = Computer::new();
        m.load_from_slice(&[16, 0, 0, 0]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!(4, m.counter);
    }

    #[test]
    fn it_runs_opcode_17() {
        // Configure display at M[A] with width B and height C
        let mut m = Computer::new();
        m.load_from_slice(&[17, 100, 640, 480]);

        assert_eq!(0, m.counter);
        m.step();
        assert_eq!(4, m.counter);

        assert_eq!(100, m.display_address);
        assert_eq!(640, m.display_width);
        assert_eq!(480, m.display_height);
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
