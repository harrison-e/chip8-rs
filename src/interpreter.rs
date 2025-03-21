use crate::types::*;
use rand::Rng;

/**
 * Interpreter struct.
 *
 * Holds all runtime data for the interpreter.
 * Contains the following members:
 * - `V`: CHIP-8's 16 8-bit registers, V0-VF. Note that VF doubles
 *   as a flag for some operations, so it should generally be avoided.
 * - `I`: The address register, used for memory operations.
 * - `Timers`: The delay and sound timers, respectively.
 * - `PC`: Program counter.
 * - `SP`: Stack pointer.
 * - `Stack`: The stack, used for function call returns.
 * - `RAM`: Program memory.
 * - `RNG`: Random number generator.
 */
pub struct Interpreter {
    V:      [u8; 16],
    I:      u16,
    Timers: [u8; 2],
    PC:     u16,
    SP:     u8,
    Stack:  [u16; 16],
    RAM:    [u8; 4096],
    RNG:    rand::rngs::ThreadRng;
}

impl Interpreter {
    // Create a new Interpreter struct
    pub fn new() -> Self {
        Self {
            V: [0; 16],
            I: 0,
            Timers: [0; 2],
            PC: 0,
            SP: 0,
            Stack: [0; 16],
            RAM: [0; 4096],
            RNG: rand::thread_rng(),
        }
    }
    
    // TODO: Load a CHIP-8 program
    pub fn load_program(&mut self) {
        unimplemented!()
    }
    
    // TODO: Run a loaded CHIP-8 program
    pub fn run(&mut self) { 
        unimplemented!() 
    }

    // TODO: Implement display. 


    // TODO: Instruction set

    // TODO: 00E0 - CLS
    // Clear the display.
    fn cls(&mut self) { 
        // clear display
        unimplemented!()
    }

    // 1nnn - JP addr 
    // Jump to location addr.
    fn jp_addr(&mut self, addr: u16) {
        addr &= 0x0FFF; // ensure that addr <= 4096
        self.PC = addr;
    }

    // 2nnn - CALL addr
    // Call subroutine at addr.
    fn call_addr(&mut self, addr: u16) {
        addr &= 0x0FFF; // ensure that addr <= 4096
        self.SP += 1;
        self.Stack[self.SP] = self.PC;
        self.PC = addr;
    }
    
    // 3xkk - SE Vx, byte 
    // Skip next instruction if Vx == kk.
    fn se_vx_byte(x: u8, kk: u8) {
        x &= 0x0F; // ensure that x <= 15
        if (self.V[x] == kk) {
            self.PC += 2;
            self.PC &= 0x0FFF; // cap PC at 4095
        }
    }

    // 4xkk - SNE Vx, byte 
    // Skip next instruction if Vx != kk.
    fn sne_vx_byte(&mut self, x: u8, kk: u8) {
        x &= 0x0F; // ensure that x <= 15
        if (self.V[x] != kk) {
            self.PC += 2;
            self.PC &= 0x0FFF; // cap PC at 4095
        }
    }

    // 5xy0 - SE Vx, Vy
    // Skip next instruction if Vx == Vy.
    fn se_vx_vy(&mut self, x: u8, y: u8) {
        // ensure that x, y are <= 15
        x &= 0x0F;
        y &= 0x0F;
        if (self.V[x] == self.V[y]) {
            self.PC += 2;
            self.PC &= 0x0FFF; // cap PC at 4095
        }
    }

    // 6xkk - LD Vx, byte 
    // Set Vx = kk.
    fn ld_vx_byte(&mut self, x: u8, kk: u8) {
        x &= 0x0F; // ensure that x <= 15
        self.V[x] = kk;
    }

    // 7xkk - ADD Vx, byte 
    // Set Vx = Vx + kk.
    fn add_vx_byte(&mut self, x: u8, kk: u8) {
        x &= 0x0F; // ensure that x <= 15
        self.V[x] += kk;
    }

    // 8xy0 - LD Vx, Vy
    // Set Vx = Vy.
    fn ld_vx_vy(&mut self, x: u8, y: u8) {
        // ensure that x, y are <= 15
        x &= 0x0F;
        y &= 0x0F;
        self.V[x] = self.V[y];
    }

    // 8xy1 - OR Vx, Vy
    // Set Vx = Vx | Vy.
    fn or_vx_vy(&mut self, x: u8, y: u8) {
        // ensure that x, y are <= 15
        x &= 0x0F;
        y &= 0x0F;
        self.V[x] |= self.V[y];
    }

    // 8xy2 - AND Vx, Vy
    // Set Vx = Vx & Vy.
    fn and_vx_vy(&mut self, x: u8, y: u8) {
        // ensure that x, y are <= 15
        x &= 0x0F;
        y &= 0x0F;
        self.V[x] &= self.V[y];
    }

    // 8xy3 - XOR Vx, Vy
    // Set Vx = Vx ^ Vy.
    fn xor_vx_vy(&mut self, x: u8, y: u8) {
        // ensure that x, y are <= 15
        x &= 0x0F;
        y &= 0x0F;
        self.V[x] ^= self.V[y];
    }

    // 8xy4 - ADD Vx, Vy
    // Set Vx = Vx + Vy.
    // If the result is greater than 8 bits (> 255), VF is set to 1, otherwise 0.
    fn add_vx_vy(&mut self, x: u8, y: u8) {
        // ensure that x, y are <= 15
        x &= 0x0F;
        y &= 0x0F;
        (wrapped, did_overflow) = self.V[x].overflowing_add(self.V[y]);
        self.V[0xF] = if did_overflow { 1 } else { 0 };
        self.V[x] = wrapped;
    }

    // 8xy5 - SUB Vx, Vy
    // Set Vx = Vx - Vy.
    // If Vx > Vy, then VF is set to 1, otherwise 0.
    fn sub_vx_vy(&mut self, x: u8, y: u8) {
        // ensure that x, y are <= 15
        x &= 0x0F;
        y &= 0x0F;
        self.V[0xF] = if self.V[x] > self.V[y] { 1 } else { 0 };
        self.V[x] -= self.V[y];
    }

    // 8xy6 - SHR Vx {, Vy}
    // Set Vx = Vx >> 1.
    // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
    fn shr_vx_vy(&mut self, x: u8, _y: u8) {
        // ensure that x, _y are <= 15
        x &= 0x0F;
        _y &= 0x0F;
        self.V[0xF] = if self.V[x] & 0x1 == 0x1 { 1 } else { 0 };
        self.V[x] >>= 1;
    }

    // 8xy7 - SUBN Vx, Vy
    // Set Vx = Vy - Vx.
    // If Vy > Vx, then VF is set to 1, otherwise 0.
    fn subn_vx_vy(&mut self, x: u8, y: u8) {
        // ensure that x, y are <= 15
        x &= 0x0F;
        y &= 0x0F;
        self.V[0xF] = if self.V[y] > self.V[x] { 1 } else { 0 };
        self.V[x] = self.V[y] - self.V[x];
    }

    // 8xyE - SHL Vx {, Vy}
    // Set Vx = Vx << 1.
    // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
    fn shl_vx_vy(&mut self, x: u8, _y: u8) {
        // ensure that x, y are <= 15
        x &= 0x0F;
        _y &= 0x0F;
        self.V[0xF] = if self.V[x] & 0x80 == 0x80 { 1 } else { 0 };
        self.V[x] <<= 1;
    }

    // 9xy0 - SNE Vx, Vy
    // Skip next instruction if Vx != Vy.
    fn sne_vx_vy(&mut self, x: u8, y: u8) {
        // ensure that x, y are <= 15
        x &= 0x0F;
        y &= 0x0F;
        if self.V[x] != self.V[y] {
            self.PC += 2;
        self.PC &= 0x0FFF; // cap PC at 4095
        }
    }

    // Annn - LD I, addr
    // The value at register I is set to addr.
    fn ld_i_addr(&mut self, addr: u16) {
        addr &= 0x0FFF; // ensure that addr <= 4096
        self.I = addr;
    }

    // Bnnn - JP V0, addr
    // Jump to location addr + V0.
    fn jp_v0_addr(&mut self, addr: u16) {
        addr &= 0x0FFF; // ensure that addr <= 4096
        self.PC = addr + self.V0; 
        self.PC &= 0x0FFF; // cap PC at 4095
    }

    // Cxkk - RND Vx, byte
    // Set Vx = random byte & kk.
    fn rnd_vx_byte(&mut self, x: u8, kk: u8) {
        x &= 0x0F; // ensure that x <= 15
        let r: u8 = self.RNG.gen();
        self.V[x] = r & kk; 
    }

    // TODO: Dxyn - DRW Vx, Vy, nibble
    // Display n-byte sprite starting at memory location I at (Vx, Vy).
    // Read n bytes from memory, starting at address in I. Read bytes are then
    // displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed
    // onto the existing screen. 
    // If any pixels are erased, VF is set to 1, otherwise 0.
    // If the sprite is positioned so part of it is outside the coordinates of the 
    // display, it wraps around to the opposite side of the screen.
    fn drw_vx_vy_n(&mut self, x: u8, y: u8, n: u8) {
        // ensure that x, y, and n are <= 15
        x &= 0x0F;
        y &= 0x0F;
        n &= 0x0F;
        // write to display
    }

    // TODO: Ex9E - SKP Vx 
    // Skip next instruction if key with the value of Vx is pressed.
    fn skp_vx(&mut self, x: u8) {
        x &= 0x0F; // ensure that x <= 15
        // check keyboard
        unimplemented!()
    }

    // TODO: ExA1 - SNKP Vx
    // Skip next instruction if key with the value of Vx is not pressed.
    fn sknp_vx(&mut self, x: u8) {
        x &= 0x0F; // ensure that x <= 15
        // check keyboard
        unimplemented!()
    }

    // Fx07 - LD Vx, DT 
    // Set Vx = delay timer value.
    fn ld_vx_dt(&mut self, x: u8) {
        x &= 0x0F; // ensure that x <= 15
        self.V[x] = self.Timers[0];
    }

    // TODO: Fx0A - LD Vx, K 
    // Wait for a key press, store the key value in Vx.
    fn ld_vx_k(&mut self, x: u8) {
        x &= 0x0F; // ensure that x <= 15
        // wait for keystroke
        // set Vx to K
        unimplemented!()
    }

    // Fx15 - LD DT, Vx 
    // Set delay timer = Vx.
    fn ld_dt_vx(&mut self, x: u8) {
        x &= 0x0F; // ensure that x <= 15
        self.Timers[0] = self.V[x];
    }

    // Fx18 - LD ST, Vx
    // Set sound timer = Vx.
    fn ld_st_vx(&mut self, x: u8) {
        x &= 0x0F; // ensure that x <= 15
        self.Timers[1] = self.V[x];
    }

    // Fx1E - ADD I, Vx 
    // Set I = I + Vx.
    fn add_i_vx(&mut self, x: u8) {
        x &= 0x0F; // ensure that x <= 15
        self.I += self.V[x];
        self.I &= 0x0FFF; // ensure that I <= 4095
    }

    // TODO: Fx29 - LD F, Vx 
    // Set I = location of sprite for digit Vx.
    fn ld_f_vx(&mut self, x: u8) {
        // TODO need to load sprites into memory
        unimplemented!()
    }

    // TODO: Fx33- LD B, Vx 
    // Store BCD representation of Vx in memory locations I, I+1, and I+2.
    fn ld_b_vx(&mut self, x: u8) {
        x &= 0x0F; // enure that x <= 15
        // TODO figure out BCD
        unimplemented!()
    }
    
    // TODO: Fx55 - LD [I], Vx 
    // Store registers V0 through Vx in memory, starting at location I.
    fn ld_i_vx(&mut self, x: u8) {
        x &= 0x0F; // ensure that x <= 15 
        // TODO figure out wraparound
        unimplemented!()
    }

    // Fx65 - LD Vx, [I]
    // Read registers V0 through Vx from memory, starting at location I.
    fn ld_vx_i(&mut self, x: u8) {
        x &= 0x0F; // ensure that x <= 15 
        // TODO figure out wraparound 
        unimplemented!()
    }
}
