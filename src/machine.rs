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

    pub fn loc(&self) -> u32 {
        self.counter
    }

    pub fn dump(&self) -> &Vec<u32> {
        &self.memory
    }

    pub fn load(&mut self, data: Vec<u32>) {
        self.memory = data;
    }

    pub fn exec(&mut self, opcode: u32, a: u32, b: u32) {
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
            }

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
    fn it_can_load_data() {
        let mut m = Machine::new();
        assert_eq!(Some(&0), m.dump().first());

        m.load(vec![1]);
        assert_eq!(Some(&1), m.dump().first());
    }

    #[test]
    fn it_can_run_opcode_1() {
        let mut m = Machine::new();
        assert_eq!(0, m.loc());

        m.load(vec![0, 2]);
        m.exec(1, 1, 0);
        assert_eq!(2, m.loc());
    }

    #[test]
    fn it_runs_opcode_2_if() {
        let mut m = Machine::new();
        assert_eq!(0, m.loc());

        m.load(vec![0, 2]);
        m.exec(2, 1, 0);
        assert_eq!(2, m.loc());
    }

    #[test]
    fn it_runs_opcode_2_else() {
        let mut m = Machine::new();
        assert_eq!(0, m.loc());

        m.load(vec![1, 2]);
        m.exec(2, 1, 0);
        assert_eq!(0, m.loc());
    }
}
