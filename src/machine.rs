use std::vec::Vec;
use std::thread;
use std::sync::mpsc;

pub enum Message {
    Pause,
    Resume,
    Step,
    Inspect,
}

pub fn spawn() -> (mpsc::Sender<Message>, mpsc::Receiver<String>) {
    let (tx_message, rx_message) = mpsc::channel();
    let (tx_data, rx_data) = mpsc::channel();

    thread::spawn(move || {
        let mut computer = Machine::new();
        let mut paused = false;

        loop {
            let message = rx_message.try_recv();
            if message.is_ok() {
                match message.unwrap() {
                    Message::Pause => {
                        paused = true;
                    },

                    Message::Resume => {
                        paused = false;
                    },

                    Message::Step => {
                        paused = true;
                        computer.step();
                    },

                    Message::Inspect => {
                        let data = format!("PAUSED {}\nPC {}", paused, computer.loc());
                        tx_data.send(data).unwrap();
                    }
                }
            }

            thread::yield_now();

            if !paused {
                computer.step();
            }

        }
    });

    return (tx_message, rx_data);
}

pub struct Machine {
    memory: Vec<u32>,
    counter: u32,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            // Default to a valid program in memory.
            // This one is an infinite loop.
            memory: vec![1, 2, 0, 0],
            counter: 0,
        }
    }

    pub fn loc(&self) -> u32 {
        self.counter
    }

    pub fn dump(&self) -> &Vec<u32> {
        &self.memory
    }

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

    fn exec(&mut self, opcode: u32, a: u32, b: u32, c: u32) {
        match opcode {
            // PC <- M[A]
            1 => {
                self.counter = self.read(a);
            },

            // If M[B] = 0, then PC <- M[A]
            2 => {
                if 0 == self.read(b) {
                  self.counter = self.read(a);
                }
            },

            // M[A] <- PC
            3 => {
                let counter = self.counter;
                self.write(a, counter);
                self.counter += 4;
            },

            // M[A] <- M[B]
            4 => {
                let b = self.read(b);
                self.write(a, b);
                self.counter += 4;
            },

            // M[A] <- M[M[B]]
            5 => {
                let b = self.read(b);
                let b = self.read(b);
                self.write(a, b);
                self.counter += 4;
            },

            // M[M[B]] <- M[A]
            6 => {
                let a = self.read(a);
                let b = self.read(b);
                self.write(b, a);
                self.counter += 4;
            },

            // M[A] <- M[B] + M[C]
            7 => {
                let b = self.read(b);
                let c = self.read(c);
                self.write(a, b + c);
                self.counter += 4;
            },

            // M[A] <- M[B] - M[C]
            8 => {
                let b = self.read(b);
                let c = self.read(c);
                self.write(a, b - c);
                self.counter += 4;
            },

            // M[A] <- M[B] * M[C]
            9 => {
                let b = self.read(b);
                let c = self.read(c);
                self.write(a, b * c);
                self.counter += 4;
            },

            // M[A] <- M[B] / M[C]
            10 => {
                let b = self.read(b);
                let c = self.read(c);
                self.write(a, b / c);
                self.counter += 4;
            },

            // M[A] <- M[B] % M[C]
            11 => {
                let b = self.read(b);
                let c = self.read(c);
                self.write(a, b % c);
                self.counter += 4;
            },

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
            },

            // MA[A] <- NOT(M[B] AND M[C])
            13 => {
                let b = self.read(b);
                let c = self.read(c);
                if b > 0 && c > 0 {
                  self.write(a, 0);
                } else {
                  self.write(a, 1);
                }
                self.counter += 4;
            },

            // Unknown opcode
            _ => {
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Machine;

    #[test]
    fn it_defaults_to_an_infinite_loop() {
        let mut m = Machine::new();

        // Read initial memory and program counter.
        let memory = m.dump().to_vec();
        let counter = m.loc();

        // Step the program twice. If nothing changes, we're in a loop.
        m.step();
        assert_eq!(counter, m.loc());
        assert_eq!(&memory, m.dump());
        m.step();
        assert_eq!(counter, m.loc());
        assert_eq!(&memory, m.dump());
    }

    #[test]
    fn it_runs_opcode_1() {
        // PC <- M[A]
        let mut m = Machine { memory: vec![1, 4, 0, 0, 2], .. Machine::new() };

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(2, m.loc());
    }

    #[test]
    fn it_runs_opcode_2_then_branch() {
        // If M[B] = 0, then PC <- M[A]
        let mut m = Machine { memory: vec![2, 4, 5, 0, 2, 0], .. Machine::new() };

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(2, m.loc());
    }

    #[test]
    fn it_runs_opcode_2_else_branch() {
        // If M[B] = 0, then PC <- M[A]
        let mut m = Machine { memory: vec![2, 4, 5, 0, 2, 1], .. Machine::new() };

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(0, m.loc());
    }

    #[test]
    fn it_runs_opcode_3() {
        // M[A] <- PC
        let mut m = Machine { memory: vec![3, 4, 0, 0, 1], .. Machine::new() };

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![3, 4, 0, 0, 0], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_4() {
        // M[A] <- M[B]
        let mut m = Machine { memory: vec![4, 4, 5, 0, 6, 7], .. Machine::new() };

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![4, 4, 5, 0, 7, 7], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_5() {
        // M[A] <- M[M[B]]
        let mut m = Machine { memory: vec![5, 4, 5, 0, 6, 7, 0, 8], .. Machine::new() };

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![5, 4, 5, 0, 8, 7, 0, 8], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_6() {
        // M[M[B]] <- M[A]
        let mut m = Machine { memory: vec![6, 4, 5, 0, 8, 6, 7], .. Machine::new() };

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![6, 4, 5, 0, 8, 6, 8], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_7() {
        // M[A] <- M[B] + M[C]
        let mut m = Machine { memory: vec![7, 4, 5, 6, 0, 11, 2], .. Machine::new() };

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![7, 4, 5, 6, 13, 11, 2], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_8() {
        // M[A] <- M[B] - M[C]
        let mut m = Machine { memory: vec![8, 4, 5, 6, 0, 11, 2], .. Machine::new() };

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![8, 4, 5, 6, 9, 11, 2], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_9() {
        // M[A] <- M[B] * M[C]
        let mut m = Machine { memory: vec![9, 4, 5, 6, 0, 11, 2], .. Machine::new() };

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![9, 4, 5, 6, 22, 11, 2], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_10() {
        // M[A] <- M[B] / M[C]
        let mut m = Machine { memory: vec![10, 4, 5, 6, 0, 11, 2], .. Machine::new() };

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![10, 4, 5, 6, 5, 11, 2], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_11() {
        // M[A] <- M[B] % M[C]
        let mut m = Machine { memory: vec![11, 4, 5, 6, 0, 11, 2], .. Machine::new() };

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![11, 4, 5, 6, 1, 11, 2], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_12_then_branch() {
        // If M[B] < M[C], then M[A] <- 1, else M[A] <- 0
        let mut m = Machine { memory: vec![12, 4, 5, 6, 2, 8, 9], .. Machine::new() };

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![12, 4, 5, 6, 1, 8, 9], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_12_else_branch() {
        // If M[B] < M[C], then M[A] <- 1, else M[A] <- 0
        let mut m = Machine { memory: vec![12, 4, 5, 6, 2, 9, 8], .. Machine::new() };

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![12, 4, 5, 6, 0, 9, 8], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_13_false_false_branch() {
        // M[A] <- NOT(M[B] AND M[C])
        let mut m = Machine { memory: vec![13, 4, 5, 6, 2, 0, 0], .. Machine::new() };

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![13, 4, 5, 6, 1, 0, 0], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_13_false_true_branch() {
        // M[A] <- NOT(M[B] AND M[C])
        let mut m = Machine { memory: vec![13, 4, 5, 6, 2, 0, 1], .. Machine::new() };

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![13, 4, 5, 6, 1, 0, 1], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_13_true_false_branch() {
        // M[A] <- NOT(M[B] AND M[C])
        let mut m = Machine { memory: vec![13, 4, 5, 6, 2, 1, 0], .. Machine::new() };

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![13, 4, 5, 6, 1, 1, 0], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_13_true_true_branch() {
        // M[A] <- NOT(M[B] AND M[C])
        let mut m = Machine { memory: vec![13, 4, 5, 6, 2, 1, 1], .. Machine::new() };

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![13, 4, 5, 6, 0, 1, 1], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_provides_safe_memory_access_when_stepping() {
        let mut m = Machine { memory: vec![0], counter: 3 };

        m.step();
        assert_eq!(&vec![0, 0, 0, 0, 0, 0, 0], m.dump());
    }
}
