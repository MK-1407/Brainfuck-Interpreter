use std::io::{self, Read};

pub struct VM {
    pub memory: Vec<u8>,
    pub data_pointer: usize,
    pub instruction_pointer: usize,
    pub program: Vec<char>,
    pub jump_table: Vec<usize>
}

impl VM {
    pub fn new(program: &str) -> Result<Self, String> {
        let mut vm = Self {
            memory: vec![0; 30000],
            data_pointer: 0,
            instruction_pointer: 0,
            program: program.chars().collect(),
            jump_table: vec![0; program.len()]
        };
        vm.build_jump_table()?;
        Ok(vm)
    }
    pub fn increment(&mut self) {
        self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_add(1);
    }
    pub fn decrement(&mut self) {
        self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_sub(1);
    }
    pub fn move_right(&mut self) -> Result<(), String> {
        if self.data_pointer < self.memory.len() - 1 {
            self.data_pointer += 1;
            Ok(())
        } else {
            Err("Memory Pointer Out of Bounds".to_string())
        }
    }
    pub fn move_left(&mut self) -> Result<(), String> {
        if self.data_pointer > 0 {
            self.data_pointer -= 1;
            Ok(())
        } else {
            Err("Memory Pointer Out of Bounds".to_string())
        }
    }
    pub fn output(&self) {
        let value = self.memory[self.data_pointer] as char;
        print!("{}", value);
    }
    pub fn input(&mut self) {
        let mut buffer = [0u8; 1];
        io::stdin().read_exact(&mut buffer).unwrap();
        self.memory[self.data_pointer] = buffer[0];
    }

    pub fn jump_forward(&mut self) {
        if self.memory[self.data_pointer] == 0 {
            self.instruction_pointer = self.jump_table[self.instruction_pointer];
        }
    }

    pub fn jump_backward(&mut self) {
        if self.memory[self.data_pointer] != 0 {
            self.instruction_pointer = self.jump_table[self.instruction_pointer] - 1;
        }
    }

    pub fn execute_instruction(&mut self, instruction: char) -> Result<(), String> {
        match instruction {
            '+' => self.increment(),
            '-' => self.decrement(),
            '>' => self.move_right()?,
            '<' => self.move_left()?,
            '.' => self.output(),
            ',' => self.input(),
            '[' => self.jump_forward(),
            ']' => self.jump_backward(),
            _ => {}
        }
        Ok(())
    }
    pub fn run(&mut self) -> Result<(), String> {
        while self.instruction_pointer < self.program.len() {
            self.execute_instruction(self.program[self.instruction_pointer])?;
            self.instruction_pointer += 1;
        }
        Ok(())
    }
    pub fn build_jump_table(&mut self) -> Result<(), String> {
        let mut stack: Vec<usize> = Vec::new();
        for (index, instruction) in self.program.iter().enumerate() {
            match instruction {
                '[' => {
                    stack.push(index);
                },
                ']' => {
                    let opening_index = match stack.pop() {
                        Some(index) => index,
                        None => return Err("Unmatched closing bracket".to_string()),
                    };
                    self.jump_table[index] = opening_index;
                    self.jump_table[opening_index] = index;
                },
                _ => {}
            }
        }
        if !stack.is_empty() {
            return Err("Unmatched opening bracket".to_string());
        }
        Ok(())
    }
}
