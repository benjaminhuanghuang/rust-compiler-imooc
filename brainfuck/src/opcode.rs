// >	Increment the data pointer (to point to the next cell to the right).
// <	Decrement the data pointer (to point to the next cell to the left).
// +	Increment (increase by one) the byte at the data pointer.
// -	Decrement (decrease by one) the byte at the data pointer.
// .	Output the byte at the data pointer.
// ,	Accept one byte of input, storing its value in the byte at the data pointer.
// [	If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
// ]	If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.
pub enum Opcode {
  SHR = 0x3E, // >
  SHL = 0x3C, // <
  ADD = 0x2B,
  SUB = 0x2D,
  PUTCHAR = 0x2E,
  GETCHAR = 0x2C,
  LB = 0x5B,
  RB = 0x5D,
}

impl From<u8> for Opcode {
  fn form(u: u8) -> Self {
    match u {
      0x3E => Opcode::SHR,
      0x3C => Opcode::SHL,
      0x2B => Opcode::ADD,
      0x2D => Opcode::SUB,
      0x2E => Opcode::PUTCHAR,
      0x2C => Opcode::GETCHAR,
      0x5B => Opcode::LB,
      0x5D => Opcode::RB,
      _ => unreachable!(),
    }
  }
}
