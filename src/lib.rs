pub mod rum;
pub mod bitpack;
pub use crate::bitpack::getu;

#[cfg(test)]
mod tests {
    
    #[test]
    fn test_parse_inst_a() {
        let inst_b: u32 = 0b10000000_00000000_00000000_01000111_u32;
        let inst = crate::rum::Instruction{
            opcode: crate::rum::Opcode::MapSegment,
            a: crate::bitpack::getu(inst_b, 3, 6) as u32,
            b: 0,
            c: crate::bitpack::getu(inst_b.into(), 3, 0) as u32,
            v: 0,
        };

        assert_eq!(inst.a, 1)
    }

    #[test]
    fn test_parse_inst_b() {
        let inst_b: u32 = 0b10000000_00000000_00000000_00001111_u32;
        let inst = crate::rum::Instruction{
            opcode: crate::rum::Opcode::MapSegment,
            a: 0,
            b: crate::bitpack::getu(inst_b, 3, 3) as u32,
            c: crate::bitpack::getu(inst_b.into(), 3, 0) as u32,
            v: 0,
        };

        assert_eq!(inst.b, 1)
    }

    #[test]
    fn test_parse_inst_c() {
        let inst_b: u32 = 0b10000000_00000000_00000000_00000111_u32;
        let inst = crate::rum::Instruction{
            opcode: crate::rum::Opcode::MapSegment,
            a: 0,
            b: 0,
            c: crate::bitpack::getu(inst_b.into(), 3, 0) as u32,
            v: 0,
        };

        assert_eq!(inst.c, 7)
    }

    #[test]
    fn test_parse_v() {
        let inst_b: u32 = 0b11010010_00000000_00000000_00000011_u32;
        let inst = crate::rum::Instruction{
            opcode: crate::rum::Opcode::LoadValue,
            a: 0,
            b: 0,
            c: 0,
            v: crate::bitpack::getu(inst_b, 25, 0) as u32,
        };

        assert_eq!(inst.v, 3)
    }

    #[test]
    fn test_map() {
        let inst_b: u32 = 0b10000000_00000000_00000000_00001111_u32;
        let inst = crate::rum::Instruction{
            opcode: crate::rum::Opcode::MapSegment,
            a: 0,
            b: 1,
            c: crate::bitpack::getu(inst_b.into(), 3, 0) as u32,
            v: 0,
        };
        let mut inst_in_mem = vec![];
        inst_in_mem.push(inst_b);
        let mut um = crate::rum::UM::new(inst_in_mem);
        um.regs[7] = 4;
        um.map_s(inst);

        assert_eq!(um.heap[1].len(), 4)
    }

    #[test]
    fn test_store() {
        let inst_b: u32 = 0b10000000_00000000_00000000_00001111_u32;
        let inst = crate::rum::Instruction{
            opcode: crate::rum::Opcode::MapSegment,
            a: 0,
            b: 1,
            c: crate::bitpack::getu(inst_b.into(), 3, 0) as u32,
            v: 0,
        };
        let inst2 = crate::rum::Instruction{
            opcode: crate::rum::Opcode::Store,
            a: 1,
            b: 2,
            c: crate::bitpack::getu(inst_b.into(), 3, 0) as u32,
            v: 0,
        };
        let mut inst_in_mem = vec![];
        inst_in_mem.push(inst_b);
        let mut um = crate::rum::UM::new(inst_in_mem);
        um.regs[7] = 4;
        um.regs[1] = 1;
        um.regs[2] = 3;
        um.map_s(inst);
        um.s_store(inst2);

        assert_eq!(um.heap[1][3], 4);
    }

    #[test]
    fn test_nand() {
        let inst_b: u32 = 0b01100000_00000000_00000000_01011011_u32;
        let inst = crate::rum::Instruction{
            opcode: crate::rum::Opcode::Nand,
            a: 1,
            b: 3,
            c: 3,
            v: 0
        };
        let mut inst_in_mem = vec![];
        inst_in_mem.push(inst_b);
        let mut um = crate::rum::UM::new(inst_in_mem);

        um.regs[inst.b as usize] = 255; 
        um.nand(inst);

        assert_eq!(um.regs[1], 0b11111111_11111111_11111111_00000000_u32);

    }

    #[test]
    fn test_load(){
        let inst_b: u32 = 0b11010010_00000000_00000000_00000011_u32;
        let inst = crate::rum::Instruction{
            opcode: crate::rum::Opcode::LoadValue,
            a: 1,
            b: 0,
            c: 0,
            v: crate::bitpack::getu(inst_b, 25, 0) as u32,
        };
        
        let mut insts = vec![];
        insts.push(inst_b);
        let mut um = crate::rum::UM::new(insts);

        um.load_value(inst);

        assert_eq!(um.regs[1], 3);
    }

    #[test]
    fn test_unmap() {
        let inst_b: u32 = 0b10000000_00000000_00000000_00001111_u32;
        let inst = crate::rum::Instruction{
            opcode: crate::rum::Opcode::MapSegment,
            a: 0,
            b: 1,
            c: crate::bitpack::getu(inst_b.into(), 3, 0) as u32,
            v: 0,
        };
        let mut inst_in_mem = vec![];
        inst_in_mem.push(inst_b);
        let mut um = crate::rum::UM::new(inst_in_mem);
        um.regs[7] = 4;
        um.map_s(inst);

        assert_eq!(um.heap[1].len(), 4)
    }
}