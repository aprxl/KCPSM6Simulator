#[derive(Debug, Clone, Copy)]
pub enum ShiftMode {
    Carry,
    Number(u8),
    Repeat,
}
