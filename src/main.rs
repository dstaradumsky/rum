use std::convert::TryInto;
use std::time::Instant;
use std::env;
mod rum;
mod bitpack;

//Taken from the rumdump lab.
pub fn read_instructions(input: Option<&str>) -> Vec<u32> {
    let mut raw_reader: Box<dyn std::io::BufRead> = match input {
        None => Box::new(std::io::BufReader::new(std::io::stdin())),
        Some(filename) => Box::new(std::io::BufReader::new(
            std::fs::File::open(filename).unwrap(),
    )),
    };
    let mut buf = Vec::<u8>::new();
    raw_reader.read_to_end(&mut buf).unwrap();

    let instructions: Vec<u32> = buf
        .chunks_exact(4)
        .map(|x| u32::from_be_bytes(x.try_into().unwrap()))
        .collect();
    instructions
}

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {eprintln!("Expected filename")};
    let filename = &args[1];
    let instructions = read_instructions(Some(filename));
    let mut universal_machine = rum::UM::new(instructions);
    let mut pc: u32 = 0;

    loop {
        let inst = rum::Instruction::new(universal_machine.get_instruction(pc));
        pc += 1;

        //println!();
        //println!("PC = {}, {:?}" , pc, inst);
        //println!("{:?}", universal_machine.regs);

        match inst.opcode {
            rum::Opcode::CMov => {universal_machine.c_move(inst)},
            rum::Opcode::Load => {universal_machine.s_load(inst)},
            rum::Opcode::Store => {universal_machine.s_store(inst)},
            rum::Opcode::Add => {universal_machine.add(inst)},
            rum::Opcode::Mul => {universal_machine.mul(inst)},
            rum::Opcode::Div => {universal_machine.div(inst)},
            rum::Opcode::Nand => {universal_machine.nand(inst)},
            rum::Opcode::Halt => {std::process::exit(0)},
            rum::Opcode::MapSegment => {universal_machine.map_s(inst)},
            rum::Opcode::UnmapSegment => {universal_machine.unmap_s(inst)},
            rum::Opcode::Output => {universal_machine.output(inst)},
            rum::Opcode::Input => {universal_machine.input(inst)},
            rum::Opcode::LoadProgram => {pc = universal_machine.load_program(inst) as u32},
            rum::Opcode::LoadValue => {universal_machine.load_value(inst)},
            rum::Opcode::Undefinied => {eprintln!("Undefined Opcode.")}
        }
    }
}
