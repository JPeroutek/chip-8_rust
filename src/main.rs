use bitmatch::bitmatch;

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
    //    Simple instructions (i.e. one-liners are handled in-line.  More complex instruction)
    #[bitmatch]
    match current_instruction {
        "0000_nnnn_nnnn_nnnn" => println!("Jmp Machine Code Routine at address: {n}"),
        "0000_0000_1110_0000" => println!("CLS"),
        "0000_0000_1110_1110" => println!("RET"),
        "0001_nnnn_nnnn_nnnn" => println!("JP\t{n:#x}"),
        "0010_nnnn_nnnn_nnnn" => println!("CALL\t{n:#x}"),
        "0011_xxxx_kkkk_kkkk" => println!("SE\tV{x:x}, {k:#x}"),
        "0100_xxxx_kkkk_kkkk" => println!("SNE\tV{x:x}, {k:#x}"),
        "0101_xxxx_yyyy_0000" => println!("SE\tV{x:x}, V{y:x}"),
        "0110_xxxx_kkkk_kkkk" => println!("LD\tV{x:x}, {k:#x}"),
        "0111_xxxx_kkkk_kkkk" => println!("ADD\tV{x:x}, {k:#x}"),
        "1000_xxxx_yyyy_0000" => println!("LD\tV{x:x}, V{y:x}"),
        "1000_xxxx_yyyy_0001" => println!("OR\tV{x:x}, V{y:x}"),
        "1000_xxxx_yyyy_0010" => println!("AND\tV{x:x}, V{y:x}"),
        "1000_xxxx_yyyy_0011" => println!("XOR\tV{x:x}, V{y:x}"),
        "1000_xxxx_yyyy_0100" => println!("ADD\tV{x:x}, V{y:x}"),
        "1000_xxxx_yyyy_0101" => println!("SUB\tV{x:x}, V{y:x}"),
        "1000_xxxx_yyyy_0110" => println!("SHR\tV{x:x}, V{y:x}"),
        "1000_xxxx_yyyy_0111" => println!("SUBN\tV{x:x}, V{y:x}"),
        "1000_xxxx_yyyy_1110" => println!("SHL\tV{x:x}, V{y:x}"),
        "1001_xxxx_yyyy_0000" => println!("SNE\tV{x:x}, V{y:x}"),
        "1010_nnnn_nnnn_nnnn" => println!("LD\tI, {n:#x}"),
        "1011_nnnn_nnnn_nnnn" => println!("JP\tV0, {n:#x}"),
        "1100_xxxx_kkkk_kkkk" => println!("RND\tV{x:x}, {k:#x}"),
        "1101_xxxx_yyyy_nnnn" => println!("DRW\tV{x:x}, V{y:x}, {n:#x}"),
        "1110_xxxx_1001_1110" => println!("SKP\tV{x:x}"),
        "1110_xxxx_1010_0001" => println!("SKNP\tV{x:x}"),
        "1111_xxxx_0000_0111" => println!("LD\tV{x:x}, DT"),
        "1111_xxxx_0000_1010" => println!("LD\tV{x:x}, K"),
        "1111_xxxx_0001_0101" => println!("LD\tDT, V{x:x}"),
        "1111_xxxx_0001_1000" => println!("LD\tST, V{x:x}"),
        "1111_xxxx_0001_1110" => println!("ADD\tI, V{x:x}"),
        "1111_xxxx_0010_1001" => println!("LD\tF, V{x:x}"),
        "1111_xxxx_0011_0011" => println!("LD\tB, V{x:x}"),
        "1111_xxxx_0101_0101" => println!("LD\t[I], V{x:x}"),
        "1111_xxxx_0110_0110" => println!("LD\tV{x:x}, [I]"),
        "????_????_????_????" => println!("Unknown OpCode.") ,
    }
}
}

fn main() {
    println!("Initializing VM");
    let mut vm_instance = Vm::new();
    vm_instance.run_program();
}
