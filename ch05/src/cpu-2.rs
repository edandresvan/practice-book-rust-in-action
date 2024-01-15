struct CPU {
  registers: [u8; 16],
  position_in_memory: usize,
  memory: [u8; 0x1000],
}

impl CPU {
  fn read_opcode(&self) -> u16 {
    let position = self.position_in_memory;
    let op_byte_1: u16 = self.memory[position] as u16;
    let op_byte_2: u16 = self.memory[position + 1] as u16;

    op_byte_1 << 8 | op_byte_2
  }

  fn run(&mut self) {
    loop {
      let opcode = self.read_opcode();
      self.position_in_memory += 2;

      let c: u8 = ((opcode & 0xf000) >> 12) as u8;
      let x: u8 = ((opcode & 0x0f00) >> 8) as u8;
      let y: u8 = ((opcode & 0x00f0) >> 4) as u8;
      let d: u8 = ((opcode & 0x000f) >> 0) as u8;

      match (c, x, y, d) {
        (0, 0, 0, 0) => {
          return;
        }
        (0x8, _, _, 0x4) => self.add_xy(x, y),
        _ => todo!("opcode {:04x}", opcode),
      }
    }
  }

  fn add_xy(
    &mut self,
    x: u8,
    y: u8,
  ) {
    let arg_1: u8 = self.registers[x as usize];
    let arg_2: u8 = self.registers[y as usize];

    let (value, overflow) = arg_1.overflowing_add(arg_2);
    self.registers[x as usize] = value;

    if overflow {
      self.registers[0xF] = 1;
    } else {
      self.registers[0xF] = 0;
    }
  }
}

fn main() {
  let mut cpu = CPU {
    registers: [0; 16],
    memory: [0; 4096],
    position_in_memory: 0,
  };

  cpu.registers[0] = 5;
  cpu.registers[1] = 10;
  cpu.registers[2] = 10;
  cpu.registers[3] = 10;

  let mem = &mut cpu.memory;
  mem[0] = 0x80;
  mem[1] = 0x14;
  mem[2] = 0x80;
  mem[3] = 0x24;
  mem[4] = 0x80;
  mem[5] = 0x34;

  cpu.run();

  assert_eq!(cpu.registers[0], 35);

  println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}
