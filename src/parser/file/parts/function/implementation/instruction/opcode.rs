#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Opcode {
    Arith,
    Jump,
    Label,
    LoadVar,
    Alloc,
    Drop,
    Invoke,
}