//スタックマシン

use crate::object::Object;

type Opcode = u8;

#[derive(Debug,Clone)]
pub enum Instruction {
    Constant(Opcode),
    Pop(Opcode),
    Push(Opcode),
    Add(Opcode,Object,Object),
}

