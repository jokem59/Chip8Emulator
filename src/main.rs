// Define Chip8 sizes
pub const MEM_LIMIT: usize = 4096;     // 4096 byte memory limit
pub const DATA_REGISTERS: usize = 16;   // 16 8 bit data registers, V0 - VF
pub const ADDRESS_REGISTER: u16 = 1; // One 16 bit wide register
pub const STACK_SIZE: usize = 48;       // 48 bytes
pub const SCREEN_WIDTH: usize = 64;  // 64 pixels wide
pub const SCREEN_HEIGHT: usize = 32; // 32 pixels wide
pub const KEYPAD_KEYS: usize = 16;   // 16 keys available on the keypad


fn main() {
    println!("Hello, world!");
}

struct Chip8 {
    op_code: u16,
    memory: [u8; MEM_LIMIT],
    cpu_register: [u8; DATA_REGISTERS],
    address_register: u16,
    program_counter: u16,
    stack: [u8; STACK_SIZE],
    gfx: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
    delay_timer: u8,
    sound_timer: u8,
    key: [u8; KEYPAD_KEYS],
}

impl Chip8 {
    pub fn new(&self) -> Chip8 {
        Chip8 {
            op_code: 0,
            memory: [0; MEM_LIMIT],
            cpu_register: [0; DATA_REGISTERS],
            address_register: 0,
            program_counter: 0,
            stack: [0; STACK_SIZE],
            gfx: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            delay_timer: 0,
            sound_timer: 0,
            key: [0; KEYPAD_KEYS],
        }
    }
}
