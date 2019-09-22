

// Define Chip8 sizes
static MEM_LIMIT: u16 = 4096;     // 4096 byte memory limit
static DATA_REGISTERS: u8 = 16;   // 16 8 bit data registers, V0 - VF
static ADDRESS_REGISTER: u16 = 1; // One 16 bit wide register
static STACK_SIZE: u8 = 48;       // 48 bytes


fn main() {

    let mut op_code: u16 = 0;     // This stores the current opcode
    
    println!("Hello, world!");
}
