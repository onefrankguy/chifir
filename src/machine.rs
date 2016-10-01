use std::vec::Vec;

pub struct Machine {
    memory: Vec<u32>,
    counter: u32,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            memory: vec![0; 2097152],
            counter: 0,
        }
    }

    pub fn with_data(data: Vec<u32>) -> Machine {
        Machine {
            memory: data,
            counter: 0,
        }
    }

    pub fn loc(&self) -> u32 {
        self.counter
    }

    pub fn dump(&self) -> &Vec<u32> {
        &self.memory
    }

    pub fn load(&mut self, data: Vec<u32>) {
        self.memory = data;
    }

    pub fn step(&mut self) {
        let counter = self.counter as usize;
        let opcode = self.memory[counter];
        let a = self.memory[counter + 1];
        let b = self.memory[counter + 2];
        let c = self.memory[counter + 3];
        self.exec(opcode, a, b, c);
    }

    pub fn exec(&mut self, opcode: u32, a: u32, b: u32, c: u32) {
        match opcode {
            // PC <- M[A]
            1 => {
                self.counter = self.memory[a as usize];
            },

            // If M[B] = 0, then PC <- M[A]
            2 => {
                if 0 == self.memory[b as usize] {
                  self.counter = self.memory[a as usize];
                }
            },

            // M[A] <- PC
            3 => {
                self.memory[a as usize] = self.counter;
                self.counter += 4;
            },

            // M[A] <- M[B]
            4 => {
                self.memory[a as usize] = self.memory[b as usize];
                self.counter += 4;
            },

            // M[A] <- M[M[B]]
            5 => {
                let b = self.memory[b as usize];
                self.memory[a as usize] = self.memory[b as usize];
                self.counter += 4;
            },

            // M[M[B]] <- M[A]
            6 => {
                let b = self.memory[b as usize];
                self.memory[b as usize] = self.memory[a as usize];
                self.counter += 4;
            },

            // M[A] <- M[B] + M[C]
            7 => {
                let b = self.memory[b as usize];
                let c = self.memory[c as usize];
                self.memory[a as usize] = b + c;
                self.counter += 4;
            },

            // M[A] <- M[B] - M[C]
            8 => {
                let b = self.memory[b as usize];
                let c = self.memory[c as usize];
                self.memory[a as usize] = b - c;
                self.counter += 4;
            },

            // M[A] <- M[B] * M[C]
            9 => {
                let b = self.memory[b as usize];
                let c = self.memory[c as usize];
                self.memory[a as usize] = b * c;
                self.counter += 4;
            },

            // M[A] <- M[B] / M[C]
            10 => {
                let b = self.memory[b as usize];
                let c = self.memory[c as usize];
                self.memory[a as usize] = b / c;
                self.counter += 4;
            },

            // M[A] <- M[B] % M[C]
            11 => {
                let b = self.memory[b as usize];
                let c = self.memory[c as usize];
                self.memory[a as usize] = b % c;
                self.counter += 4;
            },

            // If M[B] < M[C], then M[A] <- 1, else M[A] <- 0
            12 => {
                let b = self.memory[b as usize];
                let c = self.memory[c as usize];
                if b < c {
                  self.memory[a as usize] = 1;
                } else {
                  self.memory[a as usize] = 0;
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
    fn it_loads_data() {
        let mut m = Machine::new();
        assert_eq!(Some(&0), m.dump().first());

        m.load(vec![1]);
        assert_eq!(Some(&1), m.dump().first());
    }

    #[test]
    fn it_runs_opcode_1() {
        // PC <- M[A]
        let mut m = Machine::with_data(vec![1, 4, 0, 0, 2]);

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(2, m.loc());
    }

    #[test]
    fn it_runs_opcode_2_then_branch() {
        // If M[B] = 0, then PC <- M[A]
        let mut m = Machine::with_data(vec![2, 4, 5, 0, 2, 0]);

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(2, m.loc());
    }

    #[test]
    fn it_runs_opcode_2_else_branch() {
        // If M[B] = 0, then PC <- M[A]
        let mut m = Machine::with_data(vec![2, 4, 5, 0, 2, 1]);

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(0, m.loc());
    }

    #[test]
    fn it_runs_opcode_3() {
        // M[A] <- PC
        let mut m = Machine::with_data(vec![3, 4, 0, 0, 1]);

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![3, 4, 0, 0, 0], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_4() {
        // M[A] <- M[B]
        let mut m = Machine::with_data(vec![4, 4, 5, 0, 6, 7]);

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![4, 4, 5, 0, 7, 7], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_5() {
        // M[A] <- M[M[B]]
        let mut m = Machine::with_data(vec![5, 4, 5, 0, 6, 7, 0, 8]);

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![5, 4, 5, 0, 8, 7, 0, 8], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_6() {
        // M[M[B]] <- M[A]
        let mut m = Machine::with_data(vec![6, 4, 5, 0, 8, 6, 7]);

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![6, 4, 5, 0, 8, 6, 8], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_7() {
        // M[A] <- M[B] + M[C]
        let mut m = Machine::with_data(vec![7, 4, 5, 6, 0, 11, 2]);

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![7, 4, 5, 6, 13, 11, 2], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_8() {
        // M[A] <- M[B] - M[C]
        let mut m = Machine::with_data(vec![8, 4, 5, 6, 0, 11, 2]);

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![8, 4, 5, 6, 9, 11, 2], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_9() {
        // M[A] <- M[B] * M[C]
        let mut m = Machine::with_data(vec![9, 4, 5, 6, 0, 11, 2]);

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![9, 4, 5, 6, 22, 11, 2], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_10() {
        // M[A] <- M[B] / M[C]
        let mut m = Machine::with_data(vec![10, 4, 5, 6, 0, 11, 2]);

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![10, 4, 5, 6, 5, 11, 2], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_11() {
        // M[A] <- M[B] % M[C]
        let mut m = Machine::with_data(vec![11, 4, 5, 6, 0, 11, 2]);

        assert_eq!(0, m.loc());
        m.step();
        assert_eq!(&vec![11, 4, 5, 6, 1, 11, 2], m.dump());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_12_then_branch() {
        let mut m = Machine::with_data(vec![2, 3, 4]);

        assert_eq!(Some(&2), m.dump().first());
        assert_eq!(0, m.loc());
        m.exec(12, 0, 1, 2);
        assert_eq!(Some(&1), m.dump().first());
        assert_eq!(4, m.loc());
    }

    #[test]
    fn it_runs_opcode_12_else_branch() {
        let mut m = Machine::with_data(vec![2, 4, 3]);

        assert_eq!(Some(&2), m.dump().first());
        assert_eq!(0, m.loc());
        m.exec(12, 0, 1, 2);
        assert_eq!(Some(&0), m.dump().first());
        assert_eq!(4, m.loc());
    }
}
