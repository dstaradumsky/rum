use std::io::Read;

#[derive(PartialEq, Debug)]
pub enum Opcode {
    CMov, 
    Load,
    Store,
    Add,
    Mul,
    Div,
    Nand,
    Halt,
    MapSegment,
    UnmapSegment,
    Output,
    Input,
    LoadProgram,
    LoadValue,
    Undefinied
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub v: u32,
}

impl Instruction {
    pub fn new(inst: u32) -> Instruction {
        let opcode_num = crate::bitpack::getu(inst, 4, 28);
        let mut opcode = Opcode::Undefinied;
        match opcode_num {
            0 => {opcode = Opcode::CMov;},
            1 => {opcode = Opcode::Load;},
            2 => {opcode = Opcode::Store;},
            3 => {opcode = Opcode::Add;},
            4 => {opcode = Opcode::Mul;},
            5 => {opcode = Opcode::Div;},
            6 => {opcode = Opcode::Nand;},
            7 => {opcode = Opcode::Halt;},
            8 => {opcode = Opcode::MapSegment;},
            9 => {opcode = Opcode::UnmapSegment;},
            10 => {opcode = Opcode::Output;},
            11 => {opcode = Opcode::Input;},
            12 => {opcode = Opcode::LoadProgram;},
            13 => {opcode = Opcode::LoadValue;},
            _ => {eprintln!("Could not read an Opcode.")},

        }
        if opcode == Opcode::LoadValue {
            Instruction {
                opcode: opcode,
                a: crate::bitpack::getu(inst, 3, 25) as u32,
                b: 0,
                c: 0, 
                v: crate::bitpack::getu(inst, 25, 0) as u32
            }
        }
        else {
            Instruction {
                opcode: opcode,
                a: crate::bitpack::getu(inst, 3, 6) as u32,
                b: crate::bitpack::getu(inst, 3, 3) as u32,
                c: crate::bitpack::getu(inst, 3, 0) as u32,
                v: 0
            }
        }
    }
}

pub struct UM {
    pub regs: [u32; 8],
    pub heap: Vec<Vec<u32>>,
    pub free_identifiers: Vec<u32> 
}

impl UM {
    /// Returns a UM with zero filled registers and empty memory.
    /// 
    /// # Arguments
    /// 
    /// * 'instructions' - A vector of u32 words containing the instructions for the machine.
    pub fn new(instructions: Vec<u32>) -> UM {
        let mut mem = vec![];
        mem.push(instructions);
        UM {regs: [0_u32; 8], heap: mem, free_identifiers: vec![] }
    }

    ///Retrieves the current instruction (located at memory.heap[0][pc]) and returns it.
    /// 
    /// # Arguments
    /// 
    /// * 'pc' - The program counter.
    pub fn get_instruction(&self, pc: u32) -> u32 {
        return self.heap[0][pc as usize];
    }
    
    /// Executes the Conditional Move Opcode. (if $r[C] 6 = 0 then $r[A] := $r[B])
    /// 
    /// # Arguments
    /// 
    /// * 'instruction' - the current instruction the program is executing.
    pub fn c_move(&mut self, instruction: Instruction) {
        if self.regs[instruction.c as usize] != 0 {
            self.regs[instruction.a as usize] = self.regs[instruction.b as usize];
        }
    }
    
    ///Retrieves the value of at the heap index pointed to by the registers B and C in the instruction, and stores it in register A.
    /// 
    /// # Arguments
    /// 
    /// * 'instruction' - The instruction word containing registers A - the destination, and registers B and C corresponding to memory location [B][C].
    pub fn s_load(&mut self, instruction: Instruction) {
        self.regs[instruction.a as usize] = self.heap[self.regs[instruction.b as usize] as usize][self.regs[instruction.c as usize] as usize];
    }

    ///Stores the value identified by register C and stores it in heap index pointed to by the resgisters A and B.__rust_force_expr!
    /// 
    /// # Arguments
    /// 
    /// * 'instruction' - The instruction word containing registers A and B corresponding to memory location [A][B], and register C which points to the value to be stored.
    pub fn s_store(&mut self, instruction: Instruction) {
        self.heap[self.regs[instruction.a as usize] as usize][self.regs[instruction.b as usize] as usize] = self.regs[instruction.c as usize];
    }

    ///Adds the value in register b to the value in register c and stores the result in register a
    /// 
    /// # Arguments
    /// 
    /// * 'instruction' - The instruction word containing registers A - the destination, and registers B and C corresponding to memory location [B][C].
    pub fn add(&mut self, instruction: Instruction) {
        let value = self.regs[instruction.b as usize].wrapping_add(self.regs[instruction.c as usize]);
        self.regs[instruction.a as usize] = value;
    }

    ///Multiplies the value in register b to the value in register c and stores the result in register a
    /// 
    /// # Arguments
    /// 
    /// * 'instruction' - The instruction word containing registers A - the destination, and registers B and C corresponding to memory location [B][C].
    pub fn mul(&mut self, instruction: Instruction) {
        let value = self.regs[instruction.b as usize].wrapping_mul(self.regs[instruction.c as usize]);
        self.regs[instruction.a as usize] =  value;
    }

    ///Divides the value in register b to the value in register c and stores the result in register a
    /// 
    /// # Arguments
    /// 
    /// * 'instruction' - The instruction word containing registers A - the destination, and registers B and C corresponding to memory location [B][C].
    pub fn div(&mut self, instruction: Instruction) {
        let value = self.regs[instruction.b as usize] / (self.regs[instruction.c as usize]);
        self.regs[instruction.a as usize] =  value;
    }

    ///Sets register[a] to the operation !(registers[b] & registers[c])
    /// 
    /// # Arguments
    /// 
    /// * 'instruction' - The instruction word containing registers A - the destination, and registers B and C corresponding to memory location [B][C].
    pub fn nand(&mut self, instruction: Instruction) {
        let v = !(self.regs[instruction.b as usize] & self.regs[instruction.c as usize]);
        self.regs[instruction.a as usize] =  v;
    }
    
    ///Makes a new segment with a number of words equal to the value in register C and maps it in memory at heap location r[b]
    /// 
    /// # Arguments
    /// 
    /// * 'instruction' - the instruction containing the registers C containing the length of the new segment and register B which points to the location of the new segment.
    pub fn map_s(&mut self, instruction: Instruction) {
        let new_segment = vec![0 as u32; self.regs[instruction.c as usize] as usize];
        
        if self.free_identifiers.len() == 0 {
            self.heap.push(new_segment);
            self.regs[instruction.b as usize] = (self.heap.len() - 1) as u32;
        } else {
            //We do have a free seg
            let address = self.free_identifiers[self.free_identifiers.len() - 1];
            self.heap[address as usize] = new_segment;
            self.free_identifiers.pop();
            self.regs[instruction.b as usize] = address;
        }
    }

    ///The segment at memory location c is unmapped
    /// 
    /// # Arguments
    /// 
    /// * '' - the instruction containing register C which points to the segment to be unmapped
    pub fn unmap_s(&mut self, instruction: Instruction) {
        //push indentifier
        let temp = self.regs[instruction.c as usize];
        self.free_identifiers.push(temp);
        //self.heap[address as usize].clear();
        self.heap[temp as usize] = vec![];
    }
    
    ///Outputs the value in register c
    /// 
    /// # Arguments
    /// 
    /// * 'instruction' - The instruction word containing registers A - the destination, and registers B and C corresponding to memory location [B][C].
    pub fn output(&mut self, instruction: Instruction) {
        if self.regs[instruction.c as usize] > 255 {
            return;
        }
        print!("{}", (self.regs[instruction.c as usize] as u8) as char);
    }

    ///Takes the value inputted and places it in register c
    /// 
    /// # Arguments
    /// 
    /// * 'instruction' - The instruction word containing registers A - the destination, and registers B and C corresponding to memory location [B][C].
    pub fn input(&mut self, instruction: Instruction) {
        /*
        let mut line = String::new();
        std::io::stdin().read_line(&mut line);
        let trim = line.trim().to_string();
        match trim.parse::<u32>() {
            Ok(input) => {
                if input > 255 {
                    self.regs[instruction.c as usize] =  !0 as u32;
                    return;
                }
                self.regs[instruction.c as usize] =  input as u32;
                return;
            },
            Err(..) => println!("Not an integer")
        };
        */

        match std::io::stdin().bytes().next() {
            Some(value) => {
                self.regs[instruction.c as usize] = value.unwrap() as u32;
            }
            None => self.regs[instruction.c as usize] = !0 as u32,
        }
    } 

    ///Segment in the memory at the value in register B is duplicated and replaces the memory segment at location 0. The programmer count is set to m[0][C].
    /// 
    /// # Arguments
    /// 
    /// * 'instruction' - The instruction word containing registers A - the destination, and registers B and C corresponding to memory location [B][C].
    pub fn load_program(&mut self, instruction: Instruction) -> usize {
    
        if self.regs[instruction.b as usize] != 0 {
            let temp = self.heap[self.regs[instruction.b as usize] as usize].clone();
            self.heap[0] = temp;
        }
        
        return self.regs[instruction.c as usize] as usize;

    }

    ///Loads the last 25 bits of the instruction into registers[a].
    /// 
    /// # Arguments
    /// 
    /// * 'instruction' - The instruction word containing registers A - the destination, and registers B and C corresponding to memory location [B][C]. 
    pub fn load_value(&mut self, instruction: Instruction) {
        self.regs[instruction.a as usize] =  instruction.v;
    }
}