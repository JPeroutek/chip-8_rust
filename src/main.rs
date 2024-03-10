use bitmatch::bitmatch;
use rand::Rng;

struct Vm {
    memory: [u8; 4096],
    registers: [u8; 16],
    i: u16,
    pc: u16,
    sp: u8,
    st: u8,
    dt: u8,
    running: bool,
}

impl Vm {
    fn new() -> Vm {
        Vm {
            memory: [0 as u8; 4096],
            registers: [0 as u8; 16],
            i: 0,
            pc: 0,
            sp: 0,
            st: 0,
            dt: 0,
            running: true,
        }
    }

    fn run_program(&mut self) -> u8 {
        while self.running {
            let current_instruction: u16 = ((self.memory[self.pc as usize] as u16) << 8) | (self.memory[(self.pc as usize) + 1] as u16);

            self.interpret_instruction(current_instruction);
        }
        return 0;
    }

    #[bitmatch]
fn interpret_instruction(&mut self, current_instruction: u16) {
    // Instruction parser
    //    Simple instructions are in-line, more complex functions may be split into separate fn's
    let mut should_adjust_pc = true;
    #[bitmatch]
    match current_instruction {
        "0000_nnnn_nnnn_nnnn" => {
            println!("Jmp Machine Code Routine at address: {n}");
            todo!();
        },
        "0000_0000_1110_0000" => {
            println!("CLS");
            todo!();
        },
        "0000_0000_1110_1110" => {
            println!("RET");
            todo!();
        },
        "0001_nnnn_nnnn_nnnn" => {
            println!("JP\t{n:#x}");
            self.pc = n as u16;
            should_adjust_pc = false;
        },
        "0010_nnnn_nnnn_nnnn" => {println!("CALL\t{n:#x}");},
        "0011_xxxx_kkkk_kkkk" => {
            println!("SE\tV{x:x}, {k:#x}");
            if self.registers[x as usize] == k as u8{
                self.pc += 2;
            }
        },
        "0100_xxxx_kkkk_kkkk" => {
            println!("SNE\tV{x:x}, {k:#x}");
            if self.registers[x as usize] != k as u8{
                self.pc += 2;
            }
        },
        "0101_xxxx_yyyy_0000" => {
            println!("SE\tV{x:x}, V{y:x}");
            if self.registers[x as usize] == self.registers[y as usize] {
                self.pc += 2
            }
        },
        "0110_xxxx_kkkk_kkkk" => {
            println!("LD\tV{x:x}, {k:#x}");
            self.registers[x as usize] = k as u8;
        },
        "0111_xxxx_kkkk_kkkk" => {
            println!("ADD\tV{x:x}, {k:#x}");
            self.registers[x as usize] += k as u8;
        },
        "1000_xxxx_yyyy_0000" => {
            println!("LD\tV{x:x}, V{y:x}");
            self.registers[x as usize] = self.registers[y as usize];
        },
        "1000_xxxx_yyyy_0001" => {
            println!("OR\tV{x:x}, V{y:x}");
            self.registers[x as usize] |= self.registers[y as usize];
        },
        "1000_xxxx_yyyy_0010" => {
            println!("AND\tV{x:x}, V{y:x}");
            self.registers[x as usize] &= self.registers[y as usize];
        },
        "1000_xxxx_yyyy_0011" => {
            println!("XOR\tV{x:x}, V{y:x}");
            self.registers[x as usize] ^= self.registers[y as usize];
        },
        "1000_xxxx_yyyy_0100" => {
            println!("ADD\tV{x:x}, V{y:x}");
            let res: u16 = (self.registers[x as usize] as u16) + (self.registers[y as usize] as u16);
            self.registers[0xF] = (res > 255) as u8; 
            self.registers[x as usize] = res as u8
        },
        "1000_xxxx_yyyy_0101" => {
            println!("SUB\tV{x:x}, V{y:x}");
            self.registers[0xF] = (self.registers[x as usize] > self.registers[y as usize]) as u8;
            self.registers[x as usize] = self.registers[x as usize] - self.registers[y as usize];
        },
        "1000_xxxx_yyyy_0110" => {
            println!("SHR\tV{x:x}, V{y:x}");
            self.registers[0xF] = self.registers[x as usize] & 1;  // Check least significant bit
            self.registers[x as usize] = self.registers[x as usize] >> 1;
        },
        "1000_xxxx_yyyy_0111" => {
            println!("SUBN\tV{x:x}, V{y:x}");
            self.registers[0xF] = (self.registers[y as usize] > self.registers[x as usize]) as u8;
            self.registers[x as usize] = self.registers[y as usize] - self.registers[x as usize];
        },
        "1000_xxxx_yyyy_1110" => {
            println!("SHL\tV{x:x}, V{y:x}");
            self.registers[0xF] = self.registers[x as usize] & 0b10000000; // Check most significant bit
            self.registers[x as usize] = self.registers[x as usize] << 1;
        },
        "1001_xxxx_yyyy_0000" => {
            println!("SNE\tV{x:x}, V{y:x}");
            if self.registers[x as usize] != self.registers[y as usize] {
                self.pc += 2;
                should_adjust_pc = false;
            }
        },
        "1010_nnnn_nnnn_nnnn" => {
            println!("LD\tI, {n:#x}");
            self.i = n;
        },
        "1011_nnnn_nnnn_nnnn" => {
            println!("JP\tV0, {n:#x}");
            self.pc = n + (self.registers[0] as u16);
        },
        "1100_xxxx_kkkk_kkkk" => {
            println!("RND\tV{x:x}, {k:#x}");
            self.registers[x as usize] = rand::thread_rng().gen_range(0..=255) & (k as u8);
        },
        "1101_xxxx_yyyy_nnnn" => {
            println!("DRW\tV{x:x}, V{y:x}, {n:#x}");
        },
        "1110_xxxx_1001_1110" => {
            println!("SKP\tV{x:x}");
            todo!();
        },
        "1110_xxxx_1010_0001" => {
            println!("SKNP\tV{x:x}");
            todo!();
        },
        "1111_xxxx_0000_0111" => {
            println!("LD\tV{x:x}, DT");
            self.registers[x as usize] = self.dt;
        },
        "1111_xxxx_0000_1010" => {
            println!("LD\tV{x:x}, K");
            todo!();
        },
        "1111_xxxx_0001_0101" => {
            println!("LD\tDT, V{x:x}");
            self.dt = self.registers[x as usize];
        },
        "1111_xxxx_0001_1000" => {
            println!("LD\tST, V{x:x}");
            self.st = self.registers[x as usize];
        },
        "1111_xxxx_0001_1110" => {
            println!("ADD\tI, V{x:x}");
            self.i += self.registers[x as usize] as u16;
        },
        "1111_xxxx_0010_1001" => {
            println!("LD\tF, V{x:x}");
            todo!();
        },
        "1111_xxxx_0011_0011" => {
            println!("LD\tB, V{x:x}");
            // because Vx is 8 bits, we know it will be 3 digits max.
            let mut dividend = self.registers[x as usize];
            self.memory[self.i as usize] = dividend / 100;
            dividend %= 100;
            self.memory[(self.i + 1) as usize] = dividend / 10;
            dividend %= 10;
            self.memory[(self.i + 2) as usize] = dividend;
        },
        "1111_xxxx_0101_0101" => {
            println!("LD\t[I], V{x:x}");
            for index in 0..=15 {
                self.memory[((self.i as u16) + index) as usize] = self.registers[index as usize];
            }
        },
        "1111_xxxx_0110_0110" => {
            println!("LD\tV{x:x}, [I]");
            for index in 0..=15 {
                self.registers[index as usize] = self.memory[((self.i as u16) + index) as usize];
            }
        },
        "????_????_????_????" => {
            println!("Unknown OpCode.");
            should_adjust_pc = false;
            self.running = false;
        },
    }
    if should_adjust_pc { self.pc += 1; }
}
}

fn main() {
    println!("Initializing VM");
    let mut vm_instance = Vm::new();
    vm_instance.run_program();
}
