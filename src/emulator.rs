use crate::types::*;

/**
 * Emulator struct.
 *
 * Holds all runtime data for the struct.
 * Contains the following members:
 * - `V`: CHIP-8's 16 8-bit registers, V0-VF. Note that VF doubles
 *   as a flag for some operations, so it should generally be avoided.
 * - `I`: The address register, used for memory operations.
 */
pub struct Emulator {
    V:      [u8; 16],
    I:      u16,
    Timers: [u8; 2],
    PC:     u16,
    SP:     u8,
    Stack:  [u16; 16],
    RAM:    [u8; 4096],
}

impl Emulator {
    // Create a new Emulator struct
    pub fn new() -> Self {
        Self {
            V: [0; 16],
            I: 0,
            Timers: [0; 2],
            PC: 0,
            SP: 0,
            Stack: [0; 16],
            RAM: [0; 4096],
        }
    }
    
    // TODO: Load a CHIP-8 program
    pub fn load_program(&mut self) -> () { () }
    
    // TODO: Run a loaded CHIP-8 program
    pub fn run(&mut self) -> () { () }

    // TODO: Implement display. 
    // TODO: Instruction set

    // TODO: 00E0 - CLS
    // Clear the display.
    fn clear_display(&mut self) { }

    // 1nnn - JP addr 
    // Jump to location addr.
    fn jump(&mut self, addr: u16) {
        self.PC = addr;
    }

    // 2nnn - CALL addr
    // Call subroutine at addr.
    fn call(&mut self, addr: u16) {
        self.SP += 1;
        self.Stack[self.SP] = self.PC;
        self.PC = addr;
    }
    
    // 3xkk - SE Vx, byte 
    // Skip next instruction if Vx == kk.
    fn skip_if_vx_eq_byte(x: u8, kk: u8) {
        x &= 0x0F; // ensure that x <= 15
        if (self.V[x] == kk) {
            self.PC += 2;
        }
    }

    // 4xkk - SNE Vx, byte 
    // Skip next instruction if Vx != kk.
    fn skip_if_vx_ne_byte(x: u8, kk: u8) {
        x &= 0x0F; // ensure that x <= 15
        if (self.V[x] != kk) {
            self.PC += 2;
        }
    }

    // 5xy0 - SE Vx, Vy
    // Skip next instruction if Vx == Vy.
    fn skip_if_vx_eq_vy(x: u8, y: u8) {
        // ensure that x, y are <= 15
        x &= 0x0F;
        y &= 0x0F;
        if (self.V[x] == self.V[y]) {
            self.PC += 2;
        }
    }

    // 6xkk - LD Vx, byte 
    // Set Vx = kk.
    fn load_vx_byte(x: u8, kk: u8) {
        x &= 0x0F; // ensure that x <= 15
        self.V[x] = kk;
    }

    // 7xkk - ADD Vx, byte 
    // Set Vx = Vx + kk.
    fn add_vx_byte(x: u8, kk: u8) {
        x &= 0x0F; // ensure that x <= 15
        self.V[x] += kk;
    }

    // 8xy0 - LD Vx, Vy
    // Set Vx = Vy.
    fn load_vx_vy(x: u8, y: u8) {
        // ensure that x, y are <= 15
        x &= 0x0F;
        y &= 0x0F;
        self.V[x] = self.V[y];
    }

    // 8xy1 - OR Vx, Vy
    // Set Vx = Vx | Vy.
    fn or_vx_vy(x: u8, y: u8) {
        // ensure that x, y are <= 15
        x &= 0x0F;
        y &= 0x0F;
        self.V[x] |= self.V[y];
    }

    // 8xy2 - AND Vx, Vy
    // Set Vx = Vx & Vy.
    fn and_vx_vy(x: u8, y: u8) {
        // ensure that x, y are <= 15
        x &= 0x0F;
        y &= 0x0F;
        self.V[x] &= self.V[y];
    }

    // 8xy3 - XOR Vx, Vy
    // Set Vx = Vx ^ Vy.
    fn xor_vx_vy(x: u8, y: u8) {
        // ensure that x, y are <= 15
        x &= 0x0F;
        y &= 0x0F;
        self.V[x] ^= self.V[y];
    }

    // 8xy4 - ADD Vx, Vy
    // Set Vx = Vx + Vy.
    // TODO: If the result is greater than 8 bits (> 255), VF is set to 1.
    fn add_vx_vy(x: u8, y: u8) {
        // ensure that x, y are <= 15
        x &= 0x0F;
        y &= 0x0F;
        self.V[x] += self.V[y];
    }
}
