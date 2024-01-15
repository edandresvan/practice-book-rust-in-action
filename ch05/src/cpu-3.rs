struct CPU {
  registers: [u8; 16],
  position_in_memory: usize,
  memory: [u8; 4096],
  stack: [u16; 16],
  stack_pointer: usize,
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

      let nnn = opcode & 0x0fff;

      match (c, x, y, d) {
        (0, 0, 0, 0) => {
          return;
        }
        (0, 0, 0xE, 0xE) => self.ret(),
        (0x2, _, _, _) => self.call(nnn),
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
    println!("antes: r0 = {}, r1 = {}", self.registers[0], self.registers[1]);
    self.registers[x as usize] = value;
    println!("Despues: r0 = {}, r1 = {}", self.registers[0], self.registers[1]);

    if overflow {
      self.registers[0xF] = 1;
    } else {
      self.registers[0xF] = 0;
    }
  }

  fn call(
    &mut self,
    address: u16,
  ) {
    let pointer: usize = self.stack_pointer;
    let stack = &mut self.stack;

    if pointer > stack.len() {
      panic!("Stack overflow!");
    }

    stack[pointer] = self.position_in_memory as u16;
    self.stack_pointer += 1;
    self.position_in_memory = address as usize;
  }

  fn ret(&mut self) {
    if self.stack_pointer == 0 {
      panic!("Stack overflow");
    }

    self.stack_pointer -= 1;
    let address: u16 = self.stack[self.stack_pointer];
    self.position_in_memory = address as usize;
  }
}

fn main() {
  let mut cpu = CPU {
    registers: [0; 16],
    memory: [0; 4096],
    position_in_memory: 0,
    stack: [0; 16],
    stack_pointer: 0,
  };

  cpu.registers[0] = 5;
  cpu.registers[1] = 10;

  let mem = &mut cpu.memory;

  mem[0x000] = 0x21;
  mem[0x001] = 0x00;
  mem[0x002] = 0x21;
  mem[0x003] = 0x00;
  mem[0x004] = 0x00;
  mem[0x005] = 0x00;
  mem[0x100] = 0x80;
  mem[0x101] = 0x14;
  mem[0x102] = 0x80;
  mem[0x103] = 0x14;
  mem[0x104] = 0x00;
  mem[0x105] = 0xEE;

  cpu.run();

  assert_eq!(cpu.registers[0], 45);

  println!("5 + 10 + 10 + 10 + 10 = {}", cpu.registers[0]);
}
