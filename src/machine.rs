use std::vec::Vec;

pub struct Machine {
    memory: Vec<u32>
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            memory: vec![0; 2097152]
        }
    }

    pub fn dump(&self) -> &Vec<u32> {
        &self.memory
    }

    pub fn load(&mut self, data: Vec<u32>) {
        self.memory = data;
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
}
