mod font_set;

// Define Chip8 sizes
pub const MEM_LIMIT: usize = 4096;     // 4096 byte memory limit
pub const DATA_REGISTERS: usize = 16;   // 16 8 bit data registers, V0 - VF
pub const ADDRESS_REGISTER: u16 = 1; // One 16 bit wide register
pub const STACK_SIZE: usize = 48;       // 48 bytes
pub const SCREEN_WIDTH: usize = 64;  // 64 pixels wide
pub const SCREEN_HEIGHT: usize = 32; // 32 pixels wide
pub const KEYPAD_KEYS: usize = 16;   // 16 keys available on the keypad


fn main() {
    setup_graphics();
    setup_input();

    let mut chip8 = Chip8::new();

    loop {
        // Emulates one cycle
        chip8.emulate_cycle();

        // If draw flag is set, update the screen
        if chip8.get_draw_flag() {
            draw_graphics();
        }

        // Store key press state (Press and Release)
        chip8.set_keys();
    }

}

fn setup_graphics() -> () {

}

fn setup_input() -> () {

}

fn draw_graphics() -> () {

}

struct Chip8 {
    op_code: u16,
    memory: [u8; MEM_LIMIT],
    cpu_register: [u8; DATA_REGISTERS],
    address_register: u16,
    program_counter: usize,
    stack: [u8; STACK_SIZE],
    gfx: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
    delay_timer: u8,
    sound_timer: u8,
    key: [u8; KEYPAD_KEYS],
    draw_flag: bool,
}

impl Chip8 {
    pub fn new() -> Chip8 {
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
            draw_flag: false,
        }
    }

    pub fn initialize(&mut self) -> () {
        self.program_counter = 0x0200;  // Chip8 expects ROM to be loaded here on

        // Load font set into memory
        for i in 0..font_set::FONT_SET_SIZE {
            self.memory[i] = font_set::HEX_FONT_SET[i];
        }
    }

    // Only two opcodes can set this flag
    // 0x00E0: clears the screen
    // 0xDXYN: drwas a sprite on the screen
    pub fn set_draw_flag(&mut self, setting: bool) -> () {
        self.draw_flag = setting;
    }

    pub fn get_draw_flag(&self) -> bool {
        return self.draw_flag;
    }

    // Emulates one cycle of the Chip8 CPU
    pub fn emulate_cycle(&self) -> () {
        // Fetch opcode
        // Decode opcode
        // Execute opcode
        // Update timers
    }

    pub fn load_game(&self, name: &String) -> () {

    }

    // System fetches on opcode from the memory location specified by the PC (program counter)
    // Data is stored in array of u8, each opcode is 2 bytes long
    // Need to fetch two successive bytes and merge them to get the opcode
    fn fetch_opcode(&self) -> u16 {
        let first_half_opcode: u16  = self.memory[self.program_counter] as u16;
        let second_half_opcode: u16 = self.memory[self.program_counter + 1] as u16;
        let full_opcode: u16 = (first_half_opcode  << 8) | second_half_opcode;

        return full_opcode;
    }

    // Decodes the current opcode and check the opcode table to see what it means
    fn decode_opcode(&self) -> u16 {
        return 0;
    }

    // The decoded opcode is now executable
    fn execute_opcode(&self) -> u16 {
        return 0;
    }

    fn update_timers(&self) -> () {

    }

    fn set_keys(&self) -> () {

    }
}
